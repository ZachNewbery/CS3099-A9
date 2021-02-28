use crate::DBPool;
use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct ListCommunities {
    host: Option<String>,
}

#[get("/communities")]
pub(crate) async fn list_communities(
    _pool: web::Data<DBPool>,
    _request: HttpRequest,
    _specification: web::Json<ListCommunities>,
) -> Result<HttpResponse> {
    // TODO: Implement /internal/communities (GET)

    // Return type: in discussion
    unimplemented!()
}

#[derive(Serialize, Deserialize)]
pub struct CreateCommunity {
    id: String,
    title: String,
    description: String,
}

#[post("/communities/create")]
pub(crate) async fn create_community(
    _pool: web::Data<DBPool>,
    _request: HttpRequest,
    _specification: web::Json<CreateCommunity>,
) -> Result<HttpResponse> {
    // TODO: Implement /internal/communities/create (POST)

    // Return type: none
    unimplemented!()
}

#[delete("/communities/{id}")]
pub(crate) async fn delete_community(
    _pool: web::Data<DBPool>,
    _request: HttpRequest,
    web::Path(_id): web::Path<Uuid>,
) -> Result<HttpResponse> {
    // TODO: Implement /internal/communities/id (DELETE)

    // Return type: none
    unimplemented!()
}

#[patch("/communities/{id}")]
pub(crate) async fn edit_community_details(
    _pool: web::Data<DBPool>,
    _request: HttpRequest,
    web::Path(_id): web::Path<Uuid>,
) -> Result<HttpResponse> {
    // TODO: Implement /internal/communities/id (PATCH)

    // Return type: none
    unimplemented!()
}
