use actix_web::{get, post, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageParameters {
    title: Option<String>,
    content: Option<String>,
}

#[get("/")]
pub(crate) async fn search_users(web::Path(_prefix): web::Path<String>) -> Result<HttpResponse> {
    Ok(HttpResponse::NotImplemented().finish())
}

#[get("/{id}")]
pub(crate) async fn user_by_id(web::Path(_id): web::Path<String>) -> Result<HttpResponse> {
    Ok(HttpResponse::NotImplemented().finish())
}

#[post("/{id}")]
pub(crate) async fn send_user_message(_parameters: web::Query<MessageParameters>, web::Path(_id): web::Path<String>) -> Result<HttpResponse> {
    Ok(HttpResponse::NotImplemented().finish())
}