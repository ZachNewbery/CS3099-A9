use actix_web::{get, web, HttpResponse, Result};

#[get("/hello/{name}")]
pub async fn hello(web::Path(name): web::Path<String>) -> Result<String> {
    Ok(format!("Hello {}", name))
}

#[get("/key")]
pub(crate) async fn key() -> Result<HttpResponse> {
    Ok(HttpResponse::NotImplemented().finish())
}

#[get("/discover")]
pub(crate) async fn discover() -> Result<HttpResponse> {
    Ok(HttpResponse::NotImplemented().finish())
}

pub mod communities;
pub mod posts;
pub mod schemas;
pub mod users;
