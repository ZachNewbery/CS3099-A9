use actix_web::{get, HttpResponse, web};
use actix_web::{HttpRequest, Result};

use crate::DBPool;
use crate::util::header_error::HeaderError;

#[get("/")]
pub(crate) async fn communities(
    _pool: web::Data<DBPool>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(HeaderError::MissingClientHost)?;
    // TODO: Parse the client host
    // TODO: Implement /fed/communities
    Ok(HttpResponse::NotImplemented().finish())
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
    Ok(HttpResponse::NotImplemented().finish())
}
