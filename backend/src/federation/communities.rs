use actix_web::{get, web, HttpResponse};
use actix_web::{HttpRequest, Result};

use crate::database::actions::communities::{get_communities, get_community, get_community_admins};
use crate::database::actions::post::get_top_level_posts_of_community;
use crate::database::get_conn_from_pool;

use crate::federation::schemas::{Community, User};
use crate::internal::authentication::verify_federated_request;
use crate::util::route_error::RouteError;
use crate::DBPool;
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[get("")]
pub(crate) async fn communities(
    pool: web::Data<DBPool>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    let _client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?;
    // TODO: Parse the client host
    verify_federated_request(req, payload).await?;

    let conn = get_conn_from_pool(pool.clone())?;

    let communities = web::block(move || get_communities(&conn)).await?;

    Ok(HttpResponse::Ok().json(
        communities
            .into_iter()
            .map(|c| c.name)
            .collect::<Vec<String>>(),
    ))
}

#[get("/{id}")]
pub(crate) async fn community_by_id(
    pool: web::Data<DBPool>,
    req: HttpRequest,
    payload: web::Payload,
    web::Path(id): web::Path<String>,
) -> Result<HttpResponse> {
    let _client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?;
    // TODO: Parse the client host
    verify_federated_request(req, payload).await?;

    let conn = get_conn_from_pool(pool.clone())?;

    let (community, admins) = web::block(move || {
        let community = get_community(&conn, &id)?.ok_or(diesel::NotFound)?;
        let admins = get_community_admins(&conn, &community)?;
        Ok::<(_, _), RouteError>((community, admins))
    })
    .await?;

    let admins = admins
        .into_iter()
        .map(|ud| ud.into())
        .collect::<Vec<User>>();

    Ok(HttpResponse::Ok().json(Community {
        id: community.name,
        title: community.title,
        description: community.description,
        admins,
    }))
}

#[derive(Clone, Serialize, Deserialize)]
struct PostModified {
    id: Uuid,
    #[serde(with = "ts_milliseconds")]
    modified: DateTime<Utc>,
}

#[get("/{id}/timestamps")]
pub(crate) async fn community_by_id_timestamps(
    pool: web::Data<DBPool>,
    req: HttpRequest,
    payload: web::Payload,
    web::Path(id): web::Path<String>,
) -> Result<HttpResponse> {
    let _client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?;
    // TODO: Parse the client host
    verify_federated_request(req, payload).await?;

    let conn = get_conn_from_pool(pool.clone())?;

    let posts = web::block(move || {
        let community = get_community(&conn, &id)?.ok_or(diesel::NotFound)?;
        get_top_level_posts_of_community(&conn, &community)
    })
    .await?
    .unwrap_or_default()
    .into_iter()
    .filter(|p| !p.deleted)
    .map(|p| {
        Ok(PostModified {
            id: p.uuid.parse()?,
            modified: DateTime::<Utc>::from_utc(p.modified, Utc),
        })
    })
    .collect::<Result<Vec<PostModified>, RouteError>>()?;

    Ok(HttpResponse::Ok().json(posts))
}
