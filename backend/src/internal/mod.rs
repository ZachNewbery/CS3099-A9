pub mod authentication;

use crate::DBPool;
use actix_web::{post, HttpResponse};
use actix_web::{web, HttpRequest, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewUser {
    username: String,
    email: String,
    password: String,
}

#[post("/new_user")]
pub(crate) async fn new_user(
    _pool: web::Data<DBPool>,
    _new_user: web::Json<NewUser>,
) -> Result<HttpResponse> {
    // Check email/username against database
    // Insert new record into database
    todo!("implement new user")
}

// TODO: Determine if login should be email+pw or user+pw?
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
