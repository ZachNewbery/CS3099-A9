use crate::database::actions::communities::{
    get_communities, get_community, get_community_admins, put_community, remove_community,
    set_community_admins, update_community_description, update_community_title,
};
use crate::database::get_conn_from_pool;
use crate::database::models::{DatabaseNewCommunity, DatabaseCommunity};
use crate::federation::schemas::{Community, User};
use crate::internal::authentication::{authenticate, make_federated_request};
use crate::internal::get_known_hosts;
use crate::util::route_error::RouteError;
use crate::util::HOSTNAME;
use crate::DBPool;
use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse, Result};
use diesel::Connection;
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
        .map(|c| c.title)
        .collect::<Vec<String>>();

    if let Some(_) = &query.host {
        // query host has to be our own host.
        Ok(HttpResponse::Ok().json(v_comms))
    } else {
        // else collate all communities from all known hosts
        for host in get_known_hosts().iter() {
            let mut host_comms = get_host_communities(host.to_string()).await?;
            v_comms.append(&mut host_comms);
        }

        Ok(HttpResponse::Ok().json(v_comms))
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

    if query.status().is_client_error() {
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

#[get("/communities/{id}")]
pub(crate) async fn get_community_details(
    pool: web::Data<DBPool>,
    request: HttpRequest,
    web::Path(id): web::Path<String>,
) -> Result<HttpResponse> {
    let (_, _) = authenticate(pool.clone(), request)?;

    let conn = get_conn_from_pool(pool.clone())?;
    let id2 = id.clone();
    let community = web::block(move || get_community(&conn, &id))
        .await?; // it's either not local or it doesn't exist!
    
    match community {
        None => Ok(get_community_extern(id2).await?),
        Some(cmm) => Ok(get_community_local(cmm, pool).await?),
    }
    
}

pub(crate) async fn get_community_local(
    community: DatabaseCommunity,
    pool: web::Data<DBPool>,
) -> Result<HttpResponse> {
    let conn = get_conn_from_pool(pool.clone())?;
    let cmm = community.clone();
    let admins = web::block(move || get_community_admins(&conn, &cmm))
        .await?
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

pub(crate) async fn get_community_extern(
    id: String
) -> Result<HttpResponse> {
    let mut q_string = "/fed/communities/".to_owned();
    q_string.push_str(&id); 

    let mut community: Option<Community> = None;
    for host in get_known_hosts().iter() {
        let mut query = make_federated_request(
            awc::Client::get,
            host.to_string(),
            q_string.clone(),
            "{}".to_string(),
            None,
            Option::<()>::None,
        )?
        .await
        .map_err(|_| RouteError::ActixInternal)?;
        
        if query.status().is_success() {
            let body = query.body().await?;

            let s_comm: String =
            String::from_utf8(body.to_vec()).map_err(|_| RouteError::ActixInternal)?;

            community = serde_json::from_str(&s_comm)?;
        }
    }

    if let Some(comm) = community {
        Ok(HttpResponse::Ok().json(comm))
    } else {
        Ok(HttpResponse::NotFound().finish())
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
