pub mod authentication;

use crate::database::{
    get_local_user_by_email, get_local_user_by_username, insert_new_local_user, update_session,
};
use crate::internal::authentication::{generate_session, Token};
use crate::{database, DBPool};
use actix_web::{post, HttpResponse};
use actix_web::{web, HttpRequest, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[post("/new_user")]
pub(crate) async fn new_user(
    pool: web::Data<DBPool>,
    new_user: web::Json<NewUser>,
) -> Result<HttpResponse> {
    let conn = pool
        .get()
        .map_err(|_| HttpResponse::ServiceUnavailable().finish())?;

    web::block(move || {
        // Check email and username against database
        if get_local_user_by_username(&conn, new_user.username.as_str())?
            .and_then(|_| get_local_user_by_email(&conn, new_user.email.as_str()).ok()?)
            .is_none()
        {
            // Insert new record into database
            insert_new_local_user(&conn, new_user.clone())?;
        }

        Ok::<(), diesel::result::Error>(())
    })
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().finish())
}

// We will use email + password for this
#[derive(Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginOutput {
    pub token: String,
    pub token_type: String,
}

#[post("/login")]
pub(crate) async fn login(
    pool: web::Data<DBPool>,
    login_info: web::Json<Login>,
) -> Result<HttpResponse> {
    let conn = database::get_conn_from_pool(pool.clone())?;

    // Check credentials against database
    let local_user = web::block(move || {
        get_local_user_by_email(&conn, &login_info.email)?.ok_or(diesel::NotFound)
    })
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish())?;

    let new_session = generate_session();

    // Generate JWT Token
    let token = Token::new(local_user.user_id, &new_session)
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
pub(crate) async fn logout(_request: HttpRequest) -> Result<HttpResponse> {
    // Verify token validity
    // Invalidate token by blanking out session
    todo!("implement logout function")
}
