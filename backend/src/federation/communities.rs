use actix_web::{get, web, HttpResponse};
use actix_web::{HttpRequest, Result};

use crate::database::actions::communities::{get_communities, get_community_by_id};
use crate::database::get_conn_from_pool;
use crate::database::models::{DatabaseFederatedUser, DatabaseLocalUser};
use crate::federation::schemas::{Community, User};
use crate::util::route_error::RouteError;
use crate::DBPool;
use either::Either;

#[get("/")]
pub(crate) async fn communities(pool: web::Data<DBPool>, req: HttpRequest) -> Result<HttpResponse> {
    let client_host = req
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
    _pool: web::Data<DBPool>,
    req: HttpRequest,
    web::Path(id): web::Path<String>,
) -> Result<HttpResponse> {
    let client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?;
    // TODO: Parse the client host

    let conn = get_conn_from_pool(_pool.clone())?;

    let (community, admins) = web::block(move || get_community_by_id(&conn, &id)).await?;

    let admins = admins
        .into_iter()
        .map(|(u, a)| {
            match a {
                Either::Left(l) => {
                    User {
                        id: u.username,
                        host: "REPLACE-ME.com".to_string(), // TODO: Hardcode this somewhere else!
                    }
                }
                Either::Right(f) => User {
                    id: u.username,
                    host: f.host,
                },
            }
        })
        .collect::<Vec<User>>();

    Ok(HttpResponse::Ok().json(Community {
        id: community.name,
        title: community.title,
        description: community.desc,
        admins,
    }))
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
        .ok_or(RouteError::MissingClientHost)?;
    // TODO: Parse the client host
    // TODO: Implement /fed/communities/id/timestamps
    // TODO: Define return type
    // Return type: { uuid, modified }
    Ok(HttpResponse::NotImplemented().finish())
}
