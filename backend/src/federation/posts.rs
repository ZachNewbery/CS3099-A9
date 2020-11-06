use crate::database::create_federated_post;
use crate::federation::schemas::NewPost;
use crate::DBPool;
use actix_web::Result;
use actix_web::{delete, get, post, put, web, HttpResponse};
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostsParameters {
    limit: Option<u64>,
    community: Option<String>,
    min_date: Option<NaiveDateTime>,
}

#[get("/")]
pub(crate) async fn posts(_parameters: web::Query<PostsParameters>) -> Result<HttpResponse> {
    Ok(HttpResponse::NotImplemented().finish())
}

#[post("/")]
pub(crate) async fn new_post(
    pool: web::Data<DBPool>,
    new_post: web::Json<NewPost>,
) -> Result<HttpResponse> {
    let conn = pool
        .get()
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    web::block(move || {
        create_federated_post(&conn, new_post.clone())?;
        Ok::<(), diesel::result::Error>(())
    })
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().finish())
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
