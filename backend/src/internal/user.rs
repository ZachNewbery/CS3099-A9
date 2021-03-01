use crate::database::actions::local::{
    get_local_user_by_credentials, get_local_user_by_username_email, insert_new_local_user,
    update_local_user, update_session,
};
use crate::database::get_conn_from_pool;
use crate::internal::authentication::{authenticate, generate_session, Token};
use crate::{database, DBPool};
use actix_web::{post, put, HttpResponse};
use actix_web::{web, HttpRequest};
use diesel::Connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewLocalUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[post("/new_user")]
pub(crate) async fn new_user(
    pool: web::Data<DBPool>,
    new_user: web::Json<NewLocalUser>,
) -> actix_web::Result<HttpResponse> {
    let conn = pool
        .get()
        .map_err(|_| HttpResponse::ServiceUnavailable().finish())?;

    web::block(move || {
        // Check email and username against database
        if get_local_user_by_username_email(&conn, &new_user.username, &new_user.email)?.is_none() {
            // Insert new record into database
            insert_new_local_user(&conn, new_user.clone())?;
        }

        Ok::<(), diesel::result::Error>(())
    })
    .await?;

    Ok(HttpResponse::Ok().finish())
}

// We will use email + password for this
#[derive(Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginOutput {
    pub token: String,
    pub token_type: String,
}

#[post("/login")]
pub(crate) async fn login(
    pool: web::Data<DBPool>,
    login_info: web::Json<Login>,
) -> actix_web::Result<HttpResponse> {
    let conn = database::get_conn_from_pool(pool.clone())?;

    // Check credentials against database
    let local_user = web::block(move || {
        get_local_user_by_credentials(&conn, &login_info.email, &login_info.password)
    })
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish())?
    .ok_or_else(|| HttpResponse::Unauthorized().finish())?; // User not found

    let new_session = generate_session();

    // Generate JWT Token
    let token = Token::new(local_user.id, &new_session)
        .generate_token()
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    let conn = database::get_conn_from_pool(pool)?;

    // Invalidate the old session
    web::block(move || update_session(&conn, &local_user, &new_session))
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().json(LoginOutput {
        token,
        token_type: String::from("bearer"),
    }))
}

#[post("/logout")]
pub(crate) async fn logout(
    request: HttpRequest,
    pool: web::Data<DBPool>,
) -> actix_web::Result<HttpResponse> {
    // Verify token validity
    let (_, local_user) = authenticate(pool.clone(), request)?;

    // Invalidate token by blanking out session
    let conn = get_conn_from_pool(pool)?;
    web::block(move || update_session(&conn, &local_user, ""))
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().finish())
}

#[derive(Serialize, Deserialize)]
pub struct EditProfile {
    pub password: String,
}

#[put("/edit_profile")]
pub(crate) async fn edit_profile(
    request: HttpRequest,
    edit_profile: web::Json<EditProfile>,
    pool: web::Data<DBPool>,
) -> actix_web::Result<HttpResponse> {
    // Verify token validity
    let (_, local_user) = authenticate(pool.clone(), request)?;

    let conn = get_conn_from_pool(pool.clone())?;

    let new_session = generate_session();
    let token = Token::new(local_user.id, &new_session)
        .generate_token()
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    web::block(move || {
        conn.transaction(|| {
            let u = update_local_user(&conn, local_user, &*edit_profile)?;
            update_session(&conn, &u, &new_session)
        })
    })
    .await?;

    // Return type: new token
    Ok(HttpResponse::Ok().json(LoginOutput {
        token,
        token_type: String::from("bearer"),
    }))
}
