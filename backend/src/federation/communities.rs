use actix_web::{get, web, HttpResponse};
use actix_web::{HttpRequest, Result};

use crate::database::actions::communities::get_communities;
use crate::database::get_conn_from_pool;
use crate::util::header_error::HeaderError;
use crate::DBPool;

#[get("/")]
pub(crate) async fn communities(pool: web::Data<DBPool>, req: HttpRequest) -> Result<HttpResponse> {
    let client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(HeaderError::MissingClientHost)?;
    // TODO: Parse the client host

    let conn = get_conn_from_pool(pool.clone())?;

    let communities = web::block(move || get_communities(&conn))
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?; // TODO: Error types here

    Ok(HttpResponse::Ok().json(
        communities
            .into_iter()
            .map(|c| c.title)
            .collect::<Vec<String>>(),
    ))
}

#[get("/{id}")]
pub(crate) async fn community_by_id(
    _pool: web::Data<DBPool>,
    req: HttpRequest,
    web::Path(_id): web::Path<String>,
) -> Result<HttpResponse> {
    let client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(HeaderError::MissingClientHost)?;
    // TODO: Parse the client host
    // TODO: Implement /fed/communities/id
    // Return type: Community
    Ok(HttpResponse::NotImplemented().finish())
}

#[get("/{id}/timestamps")]
pub(crate) async fn community_by_id_timestamps(
    _pool: web::Data<DBPool>,
    req: HttpRequest,
    web::Path(_id): web::Path<String>,
) -> Result<HttpResponse> {
    let client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(HeaderError::MissingClientHost)?;
    // TODO: Parse the client host
    // TODO: Implement /fed/communities/id/timestamps
    // TODO: Define return type
    // Return type: { uuid, modified }
    Ok(HttpResponse::NotImplemented().finish())
}
