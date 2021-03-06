//! Federated API endpoints for actions concerning communities
use actix_web::{get, web, HttpResponse};
use actix_web::{HttpRequest, Result};

use crate::database::actions::communities::{
    get_communities, get_community_admins, get_community_by_id,
};
use crate::database::actions::post::get_top_level_posts_of_community;
use crate::database::get_conn_from_pool;

use crate::federation::schemas::{Community, User};
use crate::internal::authentication::verify_federated_request;
use crate::util::route_error::RouteError;
use crate::DBPool;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Federated endpoint to get all of the communities hosted on our instance
#[get("")]
pub(crate) async fn communities(
    pool: web::Data<DBPool>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
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

/// Federated endpoint to retrieve a community by it's name (id as per supergroup spec)
#[get("/{id}")]
pub(crate) async fn community_by_id(
    pool: web::Data<DBPool>,
    req: HttpRequest,
    payload: web::Payload,
    web::Path(id): web::Path<String>,
) -> Result<HttpResponse> {
    verify_federated_request(req, payload).await?;

    let conn = get_conn_from_pool(pool.clone())?;

    let (community, admins) = web::block(move || {
        let community = get_community_by_id(&conn, &id)?.ok_or(diesel::NotFound)?;
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

/// Struct representing a newly updated post with its updated timestamp
#[derive(Clone, Serialize, Deserialize)]
struct PostModified {
    /// UUID of the newly updated post
    id: Uuid,
    /// Time of modification
    #[serde(with = "ts_seconds")]
    modified: DateTime<Utc>,
}

/// Federated endpoint returning all the timestamps for all the posts in a Community given its name (id as per supergroup spec)
#[get("/{id}/timestamps")]
pub(crate) async fn community_by_id_timestamps(
    pool: web::Data<DBPool>,
    req: HttpRequest,
    payload: web::Payload,
    web::Path(id): web::Path<String>,
) -> Result<HttpResponse> {
    verify_federated_request(req, payload).await?;

    let conn = get_conn_from_pool(pool.clone())?;

    let posts = web::block(move || {
        let community = get_community_by_id(&conn, &id)?.ok_or(diesel::NotFound)?;
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
