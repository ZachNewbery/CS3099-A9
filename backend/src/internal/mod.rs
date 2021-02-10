use actix_web::{post, HttpResponse};
use actix_web::{web, HttpRequest, Result};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::database::actions::local::{
    get_local_user, insert_new_local_user, login_local_user, update_session,
};
use crate::database::get_conn_from_pool;
use crate::database::models::DatabasePost;
use crate::internal::authentication::{authenticate, generate_session, Token};
use crate::{database, DBPool};

pub mod authentication;

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
        if get_local_user(&conn, &new_user.username, &new_user.email)?.is_none() {
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
) -> Result<HttpResponse> {
    let conn = database::get_conn_from_pool(pool.clone())?;

    // Check credentials against database
    let local_user =
        web::block(move || login_local_user(&conn, &login_info.email, &login_info.password))
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
pub(crate) async fn logout(request: HttpRequest, pool: web::Data<DBPool>) -> Result<HttpResponse> {
    // Verify token validity
    let (_, local_user) = authenticate(pool.clone(), request)?;

    // Invalidate token by blanking out session
    let conn = get_conn_from_pool(pool)?;
    web::block(move || update_session(&conn, &local_user, ""))
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().finish())
}

// TODO: Work with frontend team to determine more appropriate routes

// #[derive(Serialize, Deserialize, Debug, Clone)]
// #[serde(rename_all = "camelCase")]
// pub(crate) struct LocalNewPost {
//     pub title: String,
//     pub body: String,
// }
//
// #[post("/new_post")]
// pub(crate) async fn new_post_local(
//     request: HttpRequest,
//     pool: web::Data<DBPool>,
//     local_new_post: web::Json<LocalNewPost>,
// ) -> Result<HttpResponse> {
//     let (_, local_user) = authenticate(pool.clone(), request)?;
//     let conn = get_conn_from_pool(pool)?;
//
//     web::block(move || create_local_post(&conn, local_new_post.0, local_user))
//         .await
//         .map_err(|_| HttpResponse::InternalServerError().finish())?;
//
//     Ok(HttpResponse::Ok().finish())
// }

// #[post("/get_posts")]
// pub(crate) async fn get_posts(
//     request: HttpRequest,
//     pool: web::Data<DBPool>,
// ) -> Result<HttpResponse> {
//     let (_, _) = authenticate(pool.clone(), request)?;
//     let conn = get_conn_from_pool(pool)?;
//
//     Ok(HttpResponse::NotImplemented().finish())
//
// }
