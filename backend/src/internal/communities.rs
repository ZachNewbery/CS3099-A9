use crate::database::actions::communities::get_communities;
use crate::database::get_conn_from_pool;
use crate::internal::authentication::authenticate;
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
    pool: web::Data<DBPool>,
    request: HttpRequest,
    query: web::Query<ListCommunities>,
) -> Result<HttpResponse> {
    let (_, _) = authenticate(pool.clone(), request)?;

    // TODO: Replace this when we have federated functionality
    if query.host.is_some() {
        return Ok(HttpResponse::NotImplemented().finish());
    }

    let conn = get_conn_from_pool(pool.clone())?;
    let communities = web::block(move || get_communities(&conn)).await?;

    Ok(HttpResponse::Ok().json(
        communities
            .into_iter()
            .map(|c| c.title)
            .collect::<Vec<String>>(),
    ))
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
