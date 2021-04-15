//! Internal API endpoints for actions concerning communities 
use crate::database::actions::communities::{
    get_communities, get_community_admins, get_community_by_id, put_community, remove_community,
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

/// Struct representing a query to list the communities of an optionally specified host
#[derive(Clone, Serialize, Deserialize)]
pub struct ListCommunities {
    /// Optionally specified host (default lists all communities in all hosts)
    host: Option<String>,
}

/// Struct representing a community stored on a federated host
#[derive(Clone, Serialize, Deserialize)]
pub struct CommunityHost {
    /// Hostname the community is stored on
    host: String,
    /// Name of the community (id as per supergroup spec)
    id: String,
}

/// Internal endpoint to list all of the communities in the database, which can be filtered by host
#[get("/communities")]
pub(crate) async fn list_communities(
    pool: web::Data<DBPool>,
    request: HttpRequest,
    query: web::Query<ListCommunities>,
) -> Result<HttpResponse> {
    let (_, _) = authenticate(pool.clone(), request)?;

    let communities = match query.host.as_deref() {
        Some(HOSTNAME) => {
            let conn = get_conn_from_pool(pool.clone())?;
            web::block(move || get_communities(&conn))
                .await?
                .into_iter()
                .map(|c| CommunityHost {
                    id: c.name,
                    host: HOSTNAME.to_string(),
                })
                .collect::<Vec<_>>()
        }
        Some(host) => external_list_communities(host)
            .await?
            .into_iter()
            .map(|s| CommunityHost {
                host: host.to_string(),
                id: s,
            })
            .collect::<Vec<_>>(),
        None => {
            let mut communities = Vec::new();
            for host in get_known_hosts().iter() {
                communities.append(
                    &mut external_list_communities(host)
                        .await?
                        .into_iter()
                        .map(|s| CommunityHost {
                            host: host.to_string(),
                            id: s,
                        })
                        .collect(),
                )
            }
            communities
        }
    };

    Ok(HttpResponse::Ok().json(communities))
}

/// Queries external hosts to also obtain their list of communities
pub(crate) async fn external_list_communities(host: &str) -> Result<Vec<String>, RouteError> {
    let mut query = make_federated_request(
        awc::Client::get,
        host.to_string(),
        "/fed/communities".to_string(),
        "{}".to_string(),
        None,
        Option::<()>::None,
    )?
    .await
    .map_err(|_| RouteError::ActixInternal)?;

    if query.status().is_success() {
        Ok(serde_json::from_str(
            &String::from_utf8(query.body().await?.to_vec())
                .map_err(|_| RouteError::ActixInternal)?,
        )?)
    } else {
        Ok(vec![])
    }
}

/// Struct representing a query to create a new community
#[derive(Serialize, Deserialize)]
pub struct CreateCommunity {
    /// Name of the community to be created
    id: String,
    /// Title of the community
    title: String,
    /// Description of the community
    description: String,
}

/// Internal endpoint to create a new Community
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

/// Struct representing a query specifying the host of a community
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommunityDetails {
    /// Hostname the community is stored on
    host: String,
}

/// Internal endpoint to retrieve the details of a Community by its name (id as per supergroup spec)
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

/// Obtains a community that is locally stored
pub(crate) fn local_get_community(
    conn: &MysqlConnection,
    id: &str,
) -> std::result::Result<Community, RouteError> {
    let community = get_community_by_id(&conn, id)?.ok_or(RouteError::NotFound)?;
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

/// Obtains a community stored on an external host
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

/// Struct representing a query to search communities
#[derive(Serialize, Deserialize, Clone)]
pub struct SearchCommunities {
    /// Optionally specified host
    host: Option<String>,
    /// Search string
    search: String,
}

/// Internal endpoint to search through Communities with a search string, can be filtered by host
#[get("/communities-search")]
pub(crate) async fn search_communities(
    pool: web::Data<DBPool>,
    query: web::Query<SearchCommunities>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, _) = authenticate(pool.clone(), request)?;

    let communities = match query.host.as_deref() {
        Some(HOSTNAME) => {
            let conn = get_conn_from_pool(pool.clone())?;
            web::block(move || get_communities(&conn))
                .await?
                .into_iter()
                .map(|c| CommunityHost {
                    id: c.name,
                    host: HOSTNAME.to_string(),
                })
                .collect::<Vec<_>>()
        }
        Some(host) => external_list_communities(host)
            .await?
            .into_iter()
            .map(|s| CommunityHost {
                host: host.to_string(),
                id: s,
            })
            .collect::<Vec<_>>(),
        None => {
            let mut communities = Vec::new();
            for host in get_known_hosts().iter() {
                communities.append(
                    &mut external_list_communities(host)
                        .await?
                        .into_iter()
                        .map(|s| CommunityHost {
                            host: host.to_string(),
                            id: s,
                        })
                        .collect(),
                )
            }
            communities
        }
    }
    .into_iter()
    .filter(|c| c.id.contains(&query.search))
    .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(communities))
}

/// Internal endpoint to delete a Community given its name (id as per supergroup spec)
#[delete("/communities/{id}")]
pub(crate) async fn delete_community(
    pool: web::Data<DBPool>,
    request: HttpRequest,
    web::Path(id): web::Path<String>,
) -> Result<HttpResponse> {
    let (_, local_user) = authenticate(pool.clone(), request)?;

    let conn = get_conn_from_pool(pool.clone())?;
    let (community, admins) = web::block(move || {
        let community = get_community_by_id(&conn, &id)?.ok_or(RouteError::NotFound)?;
        Ok::<(_, _), RouteError>((community.clone(), get_community_admins(&conn, &community)?))
    })
    .await?;

    if !admins.into_iter().any(|a| a.0.id == local_user.user_id) {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let conn = get_conn_from_pool(pool.clone())?;
    web::block(move || conn.transaction(|| remove_community(&conn, community))).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Struct representing a query to edit a community's details
#[derive(Serialize, Deserialize, Clone)]
pub struct EditCommunity {
    /// Optional new title to be set
    title: Option<String>,
    /// Optional new description to be set
    description: Option<String>,
}

/// Internal endpoint to edit a Community given its name (id as per supergroup spec)
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
        let community = get_community_by_id(&conn, &id)?.ok_or(RouteError::NotFound)?;
        Ok::<(_, _), RouteError>((community.clone(), get_community_admins(&conn, &community)?))
    })
    .await?;

    if !admins.into_iter().any(|a| a.0.id == local_user.user_id) {
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
