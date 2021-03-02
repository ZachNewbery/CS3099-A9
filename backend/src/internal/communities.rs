use crate::database::actions::communities::{
    get_communities, get_community, get_community_admins, put_community, remove_community,
    set_community_admins, update_community_description, update_community_title,
};
use crate::database::get_conn_from_pool;
use crate::database::models::{DatabaseNewCommunity};
use crate::federation::schemas::{Community, User};
use crate::internal::authentication::authenticate;
use crate::util::route_error::RouteError;
use crate::util::HOSTNAME;
use crate::DBPool;
use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse, Result};
use diesel::Connection;
use either::Either;
use serde::{Deserialize, Serialize};

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

#[get("/communities/{id}")]
pub(crate) async fn get_community_details(
    pool: web::Data<DBPool>,
    request: HttpRequest,
    web::Path(id): web::Path<String>,
) -> Result<HttpResponse> {
    let (_, _) = authenticate(pool.clone(), request)?;

    let conn = get_conn_from_pool(pool.clone())?;
    let community = web::block(move || get_community(&conn, &id))
        .await?
        .ok_or(RouteError::NotFound)?;

    let conn = get_conn_from_pool(pool.clone())?;
    let cmm = community.clone();
    let admins = web::block(move || get_community_admins(&conn, &cmm))
        .await?
        .into_iter()
        .map(|(u, d)| match d {
            Either::Left(_) => User {
                id: u.username,
                host: HOSTNAME.to_string(),
            },
            Either::Right(r) => User {
                id: u.username,
                host: r.host,
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

#[derive(Serialize, Deserialize, Clone)]
pub struct EditCommunity {
    title: Option<String>,
    description: Option<String>,
}

#[patch("/communities/{id}")]
pub(crate) async fn edit_community_details(
    pool: web::Data<DBPool>,
    request: HttpRequest,
    web::Path(id): web::Path<String>,
    edit: web::Json<EditCommunity>,
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
    web::block(move || {
        conn.transaction(|| {
            let community = match &edit.title {
                None => community,
                Some(n) => update_community_title(&conn, community, n)?,
            };

            let _ = match &edit.description {
                None => community,
                Some(n) => update_community_description(&conn, community, n)?,
            };

            Ok::<(), diesel::result::Error>(())
        })
    })
    .await?;

    Ok(HttpResponse::Ok().finish())
}
