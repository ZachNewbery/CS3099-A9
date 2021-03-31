use crate::internal::authentication::request_wrapper;
use actix_web::{get, http, web, HttpResponse, Result};
use std::fs;

#[get("/hello/{name}")]
pub async fn hello(web::Path(name): web::Path<String>) -> Result<String> {
    Ok(format!("Hello {}: {}", name, request_wrapper().await))
}

#[get("/key")]
pub(crate) async fn key() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "application/x-pem-file")
        .body(
            fs::read_to_string("fed_auth_pub.pem")?,
        ))
}

#[get("/discover")]
pub(crate) async fn discover() -> Result<HttpResponse> {
    Ok(HttpResponse::NotImplemented().finish())
}

pub mod communities;
pub mod posts;
pub mod schemas;
pub mod users;
