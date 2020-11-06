pub mod authentication;

use actix_web::Result;
use actix_web::{post, HttpResponse};

#[post("/new_user")]
pub(crate) async fn new_user() -> Result<HttpResponse> {
    // TODO: Implement new user
    // Check email/username against database
    // Insert new record into database
    unimplemented!()
}

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
pub(crate) async fn logout() -> Result<HttpResponse> {
    // TODO: Implement logout function
    // Verify token validity
    // Invalidate token by blanking out session
    unimplemented!()
}
