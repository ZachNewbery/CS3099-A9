use actix_web::{get, web, HttpResponse};
use actix_web::{HttpRequest, Result};

use crate::database::actions::communities::{get_communities, get_community, get_community_admins};
use crate::database::actions::post::get_posts_of_community;
use crate::database::get_conn_from_pool;

use crate::federation::schemas::{Community, User};
use crate::util::route_error::RouteError;
use crate::util::HOSTNAME;
use crate::DBPool;
use chrono::NaiveDateTime;
use either::Either;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[get("/")]
pub(crate) async fn communities(pool: web::Data<DBPool>, req: HttpRequest) -> Result<HttpResponse> {
    let _client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?;
    // TODO: Parse the client host

    let conn = get_conn_from_pool(pool.clone())?;

    let communities = web::block(move || get_communities(&conn)).await?;

    Ok(HttpResponse::Ok().json(
        communities
            .into_iter()
            .map(|c| c.title)
            .collect::<Vec<String>>(),
    ))
}

#[get("/{id}")]
pub(crate) async fn community_by_id(
    pool: web::Data<DBPool>,
    req: HttpRequest,
    web::Path(id): web::Path<String>,
) -> Result<HttpResponse> {
    let _client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?;
    // TODO: Parse the client host

    let conn = get_conn_from_pool(pool.clone())?;

    let (community, admins) = web::block(move || {
        let community = get_community(&conn, &id)?.ok_or(diesel::NotFound)?;
        let admins = get_community_admins(&conn, &community)?;
        Ok::<(_, _), RouteError>((community, admins))
    })
    .await?;

    let admins = admins
        .into_iter()
        .map(|(u, x)| match x {
            Either::Left(_) => User {
                id: u.username,
                host: HOSTNAME.to_string(),
            },
            Either::Right(f) => User {
                id: u.username,
                host: f.host,
            },
        })
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
    modified: NaiveDateTime, // TODO: we have to serialise this to unix time!
}

#[get("/{id}/timestamps")]
pub(crate) async fn community_by_id_timestamps(
    pool: web::Data<DBPool>,
    req: HttpRequest,
    web::Path(id): web::Path<String>,
) -> Result<HttpResponse> {
    let _client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?;
    // TODO: Parse the client host

    let conn = get_conn_from_pool(pool.clone())?;

    let posts = web::block(move || {
        let community = get_community(&conn, &id)?.ok_or(diesel::NotFound)?;
        get_posts_of_community(&conn, &community)
    })
    .await?
    .unwrap_or_default()
    .into_iter()
    .map(|p| {
        Ok(PostModified {
            id: p.uuid.parse()?,
            modified: p.modified,
        })
    })
    .collect::<Result<Vec<PostModified>, RouteError>>()?;

    Ok(HttpResponse::Ok().json(posts))

    // Return type: PostModified
}
