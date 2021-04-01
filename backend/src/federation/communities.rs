use actix_web::Result;
use actix_web::{get, web, HttpResponse};

#[get("/")]
pub(crate) async fn communities() -> Result<HttpResponse> {
    Ok(HttpResponse::NotImplemented().finish())
}

#[get("/{id}")]
pub(crate) async fn community_by_id(web::Path(_id): web::Path<String>) -> Result<HttpResponse> {
    Ok(HttpResponse::NotImplemented().finish())
}

#[get("/{id}/timestamps")]
pub(crate) async fn community_by_id_timestamps(
    web::Path(_id): web::Path<String>,
) -> Result<HttpResponse> {
    Ok(HttpResponse::NotImplemented().finish())
}
