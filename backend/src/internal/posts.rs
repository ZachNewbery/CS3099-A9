use crate::federation::schemas::{ContentType, UpdatePost};
use crate::internal::authentication::authenticate;
use crate::internal::LocatedCommunity;
use crate::DBPool;
use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct GetPost {
    host: Option<String>,
    community: Option<String>,
}

#[get("/posts/{id}")]
pub(crate) async fn get_post(
    web::Path(_id): web::Path<Uuid>,
    _query: web::Query<GetPost>,
    pool: web::Data<DBPool>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, _local_user) = authenticate(pool.clone(), request)?;

    // TODO: Implement /internal/posts/id (GET)

    // Return type: see document
    unimplemented!()
}

#[get("/posts")]
pub(crate) async fn list_posts(
    _query: web::Query<GetPost>,
    pool: web::Data<DBPool>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, _local_user) = authenticate(pool.clone(), request)?;

    // TODO: Implement /internal/posts (GET)

    // Return type: single post
    unimplemented!()
}

#[derive(Serialize, Deserialize)]
pub struct SearchPosts {
    #[serde(flatten)]
    host_community: GetPost,
    search: String,
}

#[get("/posts/search")]
pub(crate) async fn search_posts(
    _query: web::Query<GetPost>,
    pool: web::Data<DBPool>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, _local_user) = authenticate(pool.clone(), request)?;

    // TODO: Implement /internal/posts/search (GET)

    // Return type: Vec<Posts>
    unimplemented!()
}

#[derive(Serialize, Deserialize)]
pub struct CreatePost {
    pub community: LocatedCommunity,
    pub parent: Option<Uuid>,
    pub title: Option<String>,
    pub content: Vec<ContentType>,
}

#[post("/posts/create")]
pub(crate) async fn create_post(
    pool: web::Data<DBPool>,
    _post: web::Data<CreatePost>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, _local_user) = authenticate(pool.clone(), request)?;

    // TODO: Implement /internal/posts/create (POST)

    // Return type: none
    unimplemented!()
}

#[patch("/posts/{id}")]
pub(crate) async fn edit_post(
    pool: web::Data<DBPool>,
    web::Path(_id): web::Path<Uuid>,
    _post: web::Data<UpdatePost>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, _local_user) = authenticate(pool.clone(), request)?;

    // TODO: Implement /internal/posts/id (PATCH)

    // Return type: post with updated values
    unimplemented!()
}

#[delete("/posts/{id}")]
pub(crate) async fn delete_post(
    pool: web::Data<DBPool>,
    web::Path(_id): web::Path<Uuid>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, _local_user) = authenticate(pool.clone(), request)?;

    // TODO: Implement /internal/posts/id (DELETE)

    // Return type: post with updated values
    unimplemented!()
}
