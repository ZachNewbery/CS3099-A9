use crate::internal::authentication::request_wrapper;
use actix_web::{get, http, web, HttpResponse, Result};

#[get("/hello/{name}")]
pub async fn hello(web::Path(name): web::Path<String>) -> Result<String> {
    Ok(format!("Hello {}: {}", name, request_wrapper().await))
}

#[get("/key")]
pub(crate) async fn key() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "application/x-pem-file")
        .body(
            "-----BEGIN PUBLIC KEY-----\n
    MIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAxhGEP9mv/joN3UtyV0XN
    Bgwn4N9wKB7N0IDR4dN+JHqmTitUUSbm8h3wCxScbAWhhNgrqHf61kbfmnbjKf3x
    /uj5dj1t6PuGudjInB+T5/ADw01YpjF/ASt6WqB92ch3QGvaHiomVVz5yWPQzYzz
    R6DeA8GU4f6Ha4+T8fZDAqF1dggVILzm+uZNMy7tM/8qmo2dWVgD+wF1Yjhp9PWl
    bNyJ+6hx2M0PukADlXSEaVZw4grYCSIU2SyWsNCboOdZ8qsCAwqRT3K8RunBUVgk
    2/p13iBQz+E7s7dIdOKXJKtLFtnNelZsZ9uGl9ZeJS28yR2LnhZZ/LIwzMv6RINP
    OwIDAQAB\n-----END PUBLIC KEY-----",
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
