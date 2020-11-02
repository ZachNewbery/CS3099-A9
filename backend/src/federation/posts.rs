use crate::federation::schemas::NewPost;
use actix_web::Result;
use actix_web::{delete, get, post, put, web, HttpResponse};

#[get("/")]
pub(crate) async fn posts() -> Result<HttpResponse> {
    Ok(HttpResponse::NotImplemented().finish())
}

#[post("/")]
pub(crate) async fn new_post(_post: web::Json<NewPost>) -> Result<HttpResponse> {
    Ok(HttpResponse::NotImplemented().finish())
}

#[post("/{id}")]
pub(crate) async fn post_by_id(web::Path(_id): web::Path<String>) -> Result<HttpResponse> {
    Ok(HttpResponse::NotImplemented().finish())
}

#[put("/{id}")]
pub(crate) async fn edit_post(web::Path(_id): web::Path<String>) -> Result<HttpResponse> {
    Ok(HttpResponse::NotImplemented().finish())
}

#[delete("/{id}")]
pub(crate) async fn delete_post(web::Path(_id): web::Path<String>) -> Result<HttpResponse> {
    Ok(HttpResponse::NotImplemented().finish())
}
