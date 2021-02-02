use actix_web::{get, HttpResponse, post, Result, web};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageParameters {
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SearchUsersParameters {
    prefix: String,
}

#[get("/")]
pub(crate) async fn search_users(
    query: web::Query<String>
) -> Result<HttpResponse> {
    // TODO: /fed/users/
    // Return type: Vec<String>
    Ok(HttpResponse::NotImplemented().finish())
}

#[get("/{id}")]
pub(crate) async fn user_by_id(
    web::Path(_id): web::Path<String>
) -> Result<HttpResponse> {
    // TODO: /fed/users/id (GET)
    // Return type: { id, posts }
    Ok(HttpResponse::NotImplemented().finish())
}

#[post("/{id}")]
pub(crate) async fn send_user_message(
    _parameters: web::Query<MessageParameters>,
    web::Path(_id): web::Path<String>,
) -> Result<HttpResponse> {
    // TODO: /fed/users/id (POST)
    // No return type
    Ok(HttpResponse::NotImplemented().finish())
}
