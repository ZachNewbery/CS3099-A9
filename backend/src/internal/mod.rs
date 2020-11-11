pub mod authentication;

use crate::database::{get_local_user_by_email, get_local_user_by_username, insert_new_local_user};
use crate::DBPool;
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

#[post("/login")]
pub(crate) async fn login() -> Result<HttpResponse> {
    // Check credentials against database
    // Generate new session
    // Update database session
    // Generate JWT Token
    todo!("implement login function")
}

#[post("/logout")]
pub(crate) async fn logout(_request: HttpRequest) -> Result<HttpResponse> {
    // Verify token validity
    // Invalidate token by blanking out session
    todo!("implement logout function")
}
