//! Federated endpoint implementation specified as part of the supergroup protocol.
use actix_web::{get, http, web, HttpResponse, Result};
use std::fs;

#[get("/hello/{name}")]
pub async fn hello(web::Path(name): web::Path<String>) -> Result<String> {
    Ok(format!("Hello {}", name))
}

#[get("/key")]
pub(crate) async fn key() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "application/x-pem-file")
        .body(fs::read_to_string("fed_auth_pub.pem")?))
}

#[get("/discover")]
pub(crate) async fn discover() -> Result<HttpResponse> {
    let file = fs::File::open("known_hosts.txt").expect("file should open read only");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");

    Ok(HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(json))
}

pub mod communities;
pub mod posts;
pub mod schemas;
pub mod users;
