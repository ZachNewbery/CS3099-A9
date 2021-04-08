use crate::database::actions::communities::{
    get_communities, get_community, get_community_admins, put_community, remove_community,
    set_community_admins, update_community_description, update_community_title,
};
use crate::database::get_conn_from_pool;
use crate::database::models::DatabaseNewCommunity;
use crate::federation::schemas::{Community, User};
use crate::internal::authentication::{authenticate, make_federated_request};
use crate::internal::get_known_hosts;
use crate::internal::posts::cache_federated_user;
use crate::util::route_error::RouteError;
use crate::util::HOSTNAME;
use crate::DBPool;
use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse, Result};
use diesel::{Connection, MysqlConnection};
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

    if let Some(host) = &query.host {
        // query external host if needbe
        if host != HOSTNAME {
            let host_comms = get_host_communities(host.to_string()).await?;
            return Ok(HttpResponse::Ok().json(host_comms));
        }
    }

    let conn = get_conn_from_pool(pool.clone())?;
    let communities = web::block(move || get_communities(&conn)).await?;
    let mut v_comms = communities
        .into_iter()
        .map(|c| c.name)
        .collect::<Vec<String>>();

    match &query.host {
        Some(_) => {
            // query host has to be our own host.
            Ok(HttpResponse::Ok().json(v_comms))
        }
        None => {
            // else collate all communities from all known hosts
            for host in get_known_hosts().iter() {
                let mut host_comms = get_host_communities(host.to_string()).await?;
                v_comms.append(&mut host_comms);
            }

            Ok(HttpResponse::Ok().json(v_comms))
        }
    }
}

pub(crate) async fn get_host_communities(host: String) -> Result<Vec<String>, RouteError> {
    let mut query = make_federated_request(
        awc::Client::get,
        host,
        "/fed/communities".to_string(),
        "{}".to_string(),
        None,
        Option::<()>::None,
    )?
    .await
    .map_err(|_| RouteError::ActixInternal)?;

    if !query.status().is_success() {
        Ok(Vec::new())
    } else {
        let body = query.body().await?;

        let s_hosts: String =
            String::from_utf8(body.to_vec()).map_err(|_| RouteError::ActixInternal)?;

        let hosts: Vec<String> = serde_json::from_str(&s_hosts)?;

        Ok(hosts)
    }
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

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommunityDetails {
    host: String,
}

#[get("/communities/{id}")]
pub(crate) async fn get_community_details(
    pool: web::Data<DBPool>,
    request: HttpRequest,
    web::Path(id): web::Path<String>,
    query: web::Query<CommunityDetails>,
) -> Result<HttpResponse> {
    let (_, _) = authenticate(pool.clone(), request)?;

    let community = match query.host.as_str() {
        // Check ourselves
        HOSTNAME => {
            let conn = get_conn_from_pool(pool)?;
            web::block(move || local_get_community(&conn, &id)).await?
        }
        // Check another host
        host => external_get_community(pool.clone(), &id, host).await?,
    };

    Ok(HttpResponse::Ok().json(community))
}

pub(crate) fn local_get_community(
    conn: &MysqlConnection,
    id: &str,
) -> std::result::Result<Community, RouteError> {
    let community = get_community(&conn, id)?.ok_or(RouteError::NotFound)?;
    let admins = get_community_admins(&conn, &community)?
        .into_iter()
        .map(|ud| ud.into())
        .collect::<Vec<User>>();

    Ok(Community {
        id: community.name,
        title: community.title,
        description: community.description,
        admins,
    })
}

pub(crate) async fn external_get_community(
    pool: web::Data<DBPool>,
    id: &str,
    host: &str,
) -> Result<Community, RouteError> {
    let mut query = make_federated_request(
        awc::Client::get,
        host.to_string(),
        format!("/fed/communities/{}", id),
        "{}".to_string(),
        None,
        Option::<()>::None,
    )?
    .await
    .map_err(|_| RouteError::ActixInternal)?;

    if query.status().is_success() {
        let community: Community = {
            serde_json::from_str(
                &String::from_utf8(query.body().await?.to_vec())
                    .map_err(|_| RouteError::ActixInternal)?,
            )?
        };
        let admins = community.admins.clone();
        let conn = get_conn_from_pool(pool.clone()).map_err(|_| RouteError::ActixInternal)?;

        // Cache users
        web::block(move || {
            for admin in admins {
                cache_federated_user(&conn, &admin)?;
            }
            Ok(())
        })
        .await?;
        Ok(community)
    } else {
        Err(RouteError::NotFound)
    }
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
