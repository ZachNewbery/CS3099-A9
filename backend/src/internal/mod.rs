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
    // TODO: Implement new user
    // Check email/username against database
    // Insert new record into database
    unimplemented!()
}

// TODO: Determine if login should be email+pw or user+pw?
#[post("/login")]
pub(crate) async fn login() -> Result<HttpResponse> {
    // TODO: Implement login function
    // Check credentials against database
    // Generate new session
    // Update database session
    // Generate JWT Token
    unimplemented!()
}

#[post("/logout")]
pub(crate) async fn logout(_request: HttpRequest) -> Result<HttpResponse> {
    // TODO: Implement logout function
    // Verify token validity
    // Invalidate token by blanking out session
    unimplemented!()
}
