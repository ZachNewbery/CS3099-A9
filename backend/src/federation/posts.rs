use actix_web::{delete, get, HttpRequest, post, put, web};
use actix_web::{HttpResponse, Result};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::DBPool;
use crate::federation::schemas::NewPost;
use crate::util::header_error::HeaderError;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PostsParameters {
    limit: Option<u64>,
    community: Option<String>,
    min_date: Option<NaiveDateTime>,
    author: Option<String>,
    host: Option<String>,
    parent_post: Option<Uuid>,
    include_sub_children_posts: Option<bool>,
    content_type: Option<String>, // TODO: Enumerate this
}

#[get("/")]
pub(crate) async fn posts(
    _pool: web::Data<DBPool>,
    req: HttpRequest,
    _parameters: web::Query<PostsParameters>,
) -> Result<HttpResponse> {
    let client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(HeaderError::MissingClientHost)?;
    // TODO: Parse the client host

    let user_id = req
        .headers()
        .get("User-ID")
        .ok_or(HeaderError::MissingUserID)?;
    // TODO: Parse the user id

    // TODO: Implement /fed/posts (GET)
    Ok(HttpResponse::NotImplemented().finish())
}

#[post("/")]
pub(crate) async fn new_post_federated(
    pool: web::Data<DBPool>,
    req: HttpRequest,
    new_post: web::Json<NewPost>,
) -> Result<HttpResponse> {
    let client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(HeaderError::MissingClientHost)?;
    // TODO: Parse the client host

    // TODO: Check /fed/posts (POST)
    // let conn = pool
    //     .get()
    //     .map_err(|_| HttpResponse::InternalServerError().finish())?;
    //
    // web::block(move || {
    //     create_federated_post(&conn, new_post.clone())?;
    //     Ok::<(), diesel::result::Error>(())
    // })
    // .await
    // .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::NotImplemented().finish())
}

#[post("/{id}")]
pub(crate) async fn post_by_id(
    web::Path(_id): web::Path<String>,
    pool: web::Data<DBPool>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(HeaderError::MissingClientHost)?;
    // TODO: Parse the client host

    let user_id = req
        .headers()
        .get("User-ID")
        .ok_or(HeaderError::MissingUserID)?;
    // TODO: Parse the user id

    // TODO: Implement /fed/posts/id (POST)
    Ok(HttpResponse::NotImplemented().finish())
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EditPost {
    title: String,
    content: String,
}

#[put("/{id}")]
pub(crate) async fn edit_post(
    pool: web::Data<DBPool>,
    web::Path(_id): web::Path<String>,
    edit_post: web::Json<EditPost>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(HeaderError::MissingClientHost)?;
    // TODO: Parse the client host

    let user_id = req
        .headers()
        .get("User-ID")
        .ok_or(HeaderError::MissingUserID)?;
    // TODO: Parse the user id

    // TODO: Implement /fed/posts/id (PUT)
    Ok(HttpResponse::NotImplemented().finish())
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeletePost {
    id: Uuid,
}

#[delete("/{id}")]
pub(crate) async fn delete_post(
    pool: web::Data<DBPool>,
    web::Path(_id): web::Path<String>,
    delete_post: web::Json<EditPost>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(HeaderError::MissingClientHost)?;
    // TODO: Parse the client host

    let user_id = req
        .headers()
        .get("User-ID")
        .ok_or(HeaderError::MissingUserID)?;
    // TODO: Parse the user id

    // TODO: Implement /fed/posts/id (DEL)
    Ok(HttpResponse::NotImplemented().finish())
}
