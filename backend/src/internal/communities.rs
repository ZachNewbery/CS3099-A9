use crate::database::actions::communities::{
    get_communities, get_community, get_community_admins, put_community, remove_community,
    set_community_admins,
};
use crate::database::get_conn_from_pool;
use crate::database::models::DatabaseNewCommunity;
use crate::internal::authentication::authenticate;
use crate::util::route_error::RouteError;
use crate::DBPool;
use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse, Result};
use diesel::Connection;
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
    pool: web::Data<DBPool>,
    request: HttpRequest,
    specification: web::Json<CreateCommunity>,
) -> Result<HttpResponse> {
    let (_, local_user) = authenticate(pool.clone(), request)?;

    let admins = vec![local_user];
    let new_community = DatabaseNewCommunity {
        name: specification.id.clone(),
        description: specification.description.clone(),
        title: specification.title.clone(),
    };

    let conn = get_conn_from_pool(pool.clone())?;
    web::block(move || {
        conn.transaction(|| {
            let community = put_community(&conn, new_community)?;
            set_community_admins(&conn, &community, admins)?;
            Ok::<(), diesel::result::Error>(())
        })
    })
    .await?;

    Ok(HttpResponse::Ok().finish())
}

#[delete("/communities/{id}")]
pub(crate) async fn delete_community(
    pool: web::Data<DBPool>,
    request: HttpRequest,
    web::Path(id): web::Path<String>,
) -> Result<HttpResponse> {
    let (_, local_user) = authenticate(pool.clone(), request)?;

    let conn = get_conn_from_pool(pool.clone())?;
    let (community, admins) = web::block(move || {
        let community = get_community(&conn, &id)?.ok_or(RouteError::NotFound)?;
        Ok::<(_, _), RouteError>((community.clone(), get_community_admins(&conn, &community)?))
    })
    .await?;

    if !admins.into_iter().any(|a| a.0.id == local_user.id) {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let conn = get_conn_from_pool(pool.clone())?;
    web::block(move || conn.transaction(|| remove_community(&conn, community))).await?;

    Ok(HttpResponse::Ok().finish())
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
