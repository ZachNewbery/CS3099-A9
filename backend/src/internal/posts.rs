use crate::database::actions::communities::{get_community_admins, get_community_by_id};
use crate::database::actions::post;
use crate::database::actions::post::{
    get_all_top_level_posts, get_children_posts_of, get_top_level_posts_of_community,
    modify_post_title, put_post, put_post_contents, remove_post, remove_post_contents, touch_post,
    PostInformation,
};
use crate::database::actions::user::{
    get_name_from_local_user, get_user_detail_by_name, insert_new_federated_user,
};
use crate::database::get_conn_from_pool;
use crate::database::models::{DatabaseLocalUser, DatabaseNewPost};
use crate::federation::posts::EditPost;
use crate::federation::schemas::{ContentType, DatabaseContentType, Post, User};
use crate::internal::authentication::{authenticate, make_federated_request};
use crate::internal::{get_known_hosts, LocatedCommunity};
use crate::util::route_error::RouteError;
use crate::util::HOSTNAME;
use crate::DBPool;
use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse, Result};
use awc::SendClientRequest;
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use diesel::{Connection, MysqlConnection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct GetPost {
    host: String,
    community: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LocatedPost {
    pub(crate) id: Uuid,
    pub(crate) community: LocatedCommunity,
    pub(crate) parent_post: Option<Uuid>,
    pub(crate) children: Vec<Uuid>,
    pub(crate) title: Option<String>,
    pub(crate) content: Vec<HashMap<ContentType, serde_json::Value>>,
    pub(crate) author: User,
    #[serde(with = "ts_seconds")]
    pub(crate) modified: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub(crate) created: DateTime<Utc>,
    #[serde(default)]
    pub(crate) deleted: bool,
}

#[get("/posts/{id}")]
pub(crate) async fn get_post(
    web::Path(id): web::Path<Uuid>,
    query: web::Query<GetPost>,
    pool: web::Data<DBPool>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, local_user) = authenticate(pool.clone(), request)?;
    let conn = get_conn_from_pool(pool.clone())?;

    let post = match query.host.as_str() {
        // Check local
        HOSTNAME => {
            let post = web::block(move || post::get_post(&conn, &id))
                .await?
                .ok_or(RouteError::NotFound)?;

            get_post_local(pool, post).await
        }

        // Ask the specified host
        host => {
            let u = web::block(move || get_name_from_local_user(&conn, local_user)).await?;
            external_get_post(&id, pool, &u.username, host).await
        }
    }?;
    // Return type: a monstrosity, honestly.
    Ok(HttpResponse::Ok().json(post))
}

pub(crate) async fn get_post_local(
    pool: web::Data<DBPool>,
    post: PostInformation,
) -> Result<LocatedPost, RouteError> {
    let conn = get_conn_from_pool(pool.clone()).map_err(|_| RouteError::ActixInternal)?;
    let parent = post.post.clone();
    let children = web::block(move || get_children_posts_of(&conn, &parent))
        .await
        .map_err(|_| RouteError::ActixInternal)?
        .unwrap_or_default();

    let lp = LocatedPost {
        id: post.post.uuid.parse().map_err(RouteError::UuidParse)?,
        community: LocatedCommunity::Local {
            id: post.community.name,
        },
        parent_post: post
            .parent
            .map(|u| u.uuid.parse().map_err(RouteError::UuidParse))
            .transpose()?,
        children: children
            .into_iter()
            .map(|p| Ok(p.post.uuid.parse()?))
            .collect::<Result<Vec<_>, RouteError>>()?,
        title: Some(post.post.title),
        content: post.content,
        author: (post.user, post.user_details).into(),
        modified: DateTime::<Utc>::from_utc(post.post.modified, Utc),
        created: DateTime::<Utc>::from_utc(post.post.created, Utc),
        deleted: post.post.deleted,
    };

    Ok(lp)
}

pub(crate) fn request_get_post(
    uuid: &Uuid,
    host: &str,
    username: &str,
) -> Result<SendClientRequest, RouteError> {
    make_federated_request(
        awc::Client::get,
        host.to_string(),
        format!("/fed/posts/{}", uuid.to_string()),
        "{}".to_string(),
        Some(username.to_string()),
        Option::<()>::None,
    )
}

// Gets one post matching UUID from a host.
pub(crate) async fn external_get_post(
    uuid: &Uuid,
    pool: web::Data<DBPool>,
    username: &str,
    host: &str,
) -> Result<LocatedPost, RouteError> {
    let mut query = request_get_post(&uuid, host, username)?
        .await
        .map_err(|_| RouteError::ActixInternal)?;

    if !query.status().is_success() {
        return Err(RouteError::ExternalService);
    } else {
        let post: Post = {
            let body = query.body().await?;
            serde_json::from_str(
                &String::from_utf8(body.to_vec()).map_err(|_| RouteError::ActixInternal)?,
            )?
        };

        let conn = get_conn_from_pool(pool.clone()).map_err(|_| RouteError::ActixInternal)?;
        let author = post.author.clone();
        web::block(move || {
            cache_federated_user(&conn, &author)?;
            Ok::<(), RouteError>(())
        })
        .await?;

        Ok(LocatedPost {
            id: post.id,
            community: LocatedCommunity::Federated {
                id: post.community,
                host: host.to_string(),
            },
            parent_post: post.parent_post,
            children: post.children,
            title: post.title,
            content: post.content,
            author: post.author,
            modified: post.modified,
            created: post.created,
            deleted: false,
        })
    }
}

// Writes a federated author to cache.
pub(crate) fn cache_federated_user(
    conn: &MysqlConnection,
    federated_user: &User,
) -> Result<(), diesel::result::Error> {
    match federated_user.host.as_ref() {
        HOSTNAME => Ok(()),
        _ => match get_user_detail_by_name(conn, &federated_user.id) {
            Ok(_) => Ok(()),
            Err(diesel::NotFound) => {
                insert_new_federated_user(conn, federated_user)?;
                Ok(())
            }
            Err(e) => Err(e),
        },
    }
}

#[get("/posts")]
pub(crate) async fn list_posts(
    query: web::Query<GetPost>,
    pool: web::Data<DBPool>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, local_user) = authenticate(pool.clone(), request)?;

    let conn = get_conn_from_pool(pool.clone())?;
    let user = web::block(move || get_name_from_local_user(&conn, local_user)).await?;

    let posts = match query.host.as_str() {
        // Our name
        HOSTNAME => list_local_posts(query.community.as_deref(), pool.clone()).await?,

        // Ask another host
        host => {
            external_list_posts(
                host,
                query.community.as_deref(),
                &user.username,
                pool.clone(),
            )
            .await?
        }
    }
    .into_iter()
    .filter(|p| p.parent_post.is_none())
    .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(posts))
}

pub(crate) async fn list_local_posts(
    community: Option<&str>,
    pool: web::Data<DBPool>,
) -> Result<Vec<LocatedPost>, RouteError> {
    let conn = get_conn_from_pool(pool.clone()).map_err(|_| RouteError::ActixInternal)?;
    let comm = community.map(str::to_string);
    let posts = web::block(move || {
        let posts = match &comm {
            None => get_all_top_level_posts(&conn),
            Some(c) => {
                let community = get_community_by_id(&conn, c)?.ok_or(diesel::NotFound)?;

                get_top_level_posts_of_community(&conn, &community)
            }
        }?
        .unwrap_or_default();

        posts
            .into_iter()
            .map(|p| {
                let post = post::get_post(&conn, &p.uuid.parse()?)?.ok_or(diesel::NotFound)?;

                let children = get_children_posts_of(&conn, &p)?.unwrap_or_default();

                Ok((post, children))
            })
            .collect::<Result<Vec<(PostInformation, Vec<PostInformation>)>, RouteError>>()
    })
    .await
    .map_err(|_| RouteError::ActixInternal)?;

    let posts = posts
        .into_iter()
        .map(|(p, c)| {
            Ok(LocatedPost {
                id: p.post.uuid.parse().map_err(RouteError::UuidParse)?,
                community: LocatedCommunity::Local {
                    id: p.community.name,
                },
                parent_post: None,
                children: c
                    .into_iter()
                    .map(|h| h.post.uuid.parse().map_err(RouteError::UuidParse))
                    .collect::<Result<Vec<Uuid>, RouteError>>()?,
                title: Some(p.post.title),
                content: p.content,
                author: (p.user, p.user_details).into(),
                modified: DateTime::<Utc>::from_utc(p.post.modified, Utc),
                created: DateTime::<Utc>::from_utc(p.post.created, Utc),
                deleted: p.post.deleted,
            })
        })
        .collect::<Result<Vec<LocatedPost>, RouteError>>()?;

    Ok(posts)
}

// Returns a list of all posts from one host.
pub(crate) async fn external_list_posts(
    host: &str,
    community: Option<&str>,
    requester_name: &str,
    pool: web::Data<DBPool>,
) -> Result<Vec<LocatedPost>, RouteError> {
    // If there is a community then include it in the query
    let mut query: HashMap<String, String> = HashMap::new();
    if let Some(comm) = community {
        query.insert("community".to_string(), comm.to_string());
    }

    // Turn empty queries into None
    let opt_query = if query.is_empty() { None } else { Some(query) };

    external_list_posts_inner(host, requester_name, pool, opt_query).await
}

async fn external_list_posts_inner(
    host: &str,
    requester_name: &str,
    pool: web::Data<DBPool>,
    opt_query: Option<HashMap<String, String>>,
) -> Result<Vec<LocatedPost>, RouteError> {
    let mut req = make_federated_request(
        awc::Client::get,
        host.to_string(),
        "/fed/posts".to_string(),
        "{}".to_string(),
        Some(requester_name.to_string()),
        opt_query,
    )?
    .await
    .map_err(|_| RouteError::ActixInternal)?;

    if req.status().is_success() {
        let posts: Vec<Post> = {
            let s_posts: String = String::from_utf8(req.body().await?.to_vec())
                .map_err(|_| RouteError::ActixInternal)?;
            serde_json::from_str(&s_posts).map_err(|_| RouteError::ActixInternal)?
        };

        let conn = get_conn_from_pool(pool.clone()).map_err(|_| RouteError::ActixInternal)?;
        let host2 = host.to_string();
        web::block(move || {
            posts
                .into_iter()
                .map(|p| {
                    cache_federated_user(&conn, &p.author)?;
                    Ok(LocatedPost {
                        id: p.id,
                        community: LocatedCommunity::Federated {
                            id: p.community,
                            host: (&host2).to_string(),
                        },
                        parent_post: p.parent_post,
                        children: p.children,
                        title: p.title,
                        content: p.content,
                        author: p.author,
                        modified: p.modified,
                        created: p.created,
                        deleted: false,
                    })
                })
                .collect::<Result<Vec<LocatedPost>, RouteError>>()
        })
        .await
        .map_err(RouteError::from)
    } else {
        Err(RouteError::ActixInternal)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SearchPosts {
    host: Option<String>,
    community: Option<String>,
    search: String,
}

#[get("/posts-search")]
pub(crate) async fn search_posts(
    query: web::Query<SearchPosts>,
    pool: web::Data<DBPool>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, local_user) = authenticate(pool.clone(), request)?;

    let conn = get_conn_from_pool(pool.clone())?;
    let user = web::block(move || get_name_from_local_user(&conn, local_user)).await?;

    let posts = match query.host.as_deref() {
        // Our name
        Some(HOSTNAME) => list_local_posts(query.community.as_deref(), pool.clone()).await?,

        // Ask another host
        Some(host) => {
            external_list_posts(
                host,
                query.community.as_deref(),
                &user.username,
                pool.clone(),
            )
            .await?
        }

        // No host? Ask them all
        None => {
            let mut posts = Vec::new();
            for host in get_known_hosts() {
                posts.append(
                    &mut external_list_posts(
                        &host,
                        query.community.as_deref(),
                        &user.username,
                        pool.clone(),
                    )
                    .await?,
                )
            }
            posts
        }
    }
    .into_iter()
    .filter(|p| p.parent_post.is_none())
    // Search
    .filter(|p| {
        p.content.iter().any(|c| {
            let content = if c.contains_key(&ContentType::Text) {
                c.get(&ContentType::Text)
                    .unwrap()
                    .get("text")
                    .unwrap()
                    .as_str()
            } else if c.contains_key(&ContentType::Markdown) {
                c.get(&ContentType::Markdown)
                    .unwrap()
                    .get("text")
                    .unwrap()
                    .as_str()
            } else {
                Some("")
            };
            content.unwrap().contains(&query.search)
        }) || p.title.as_ref().unwrap().contains(&query.search)
    })
    .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(posts))
}

#[derive(Serialize, Deserialize)]
pub struct CreateCommunity {
    id: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePost {
    pub community: CreateCommunity,
    pub parent: Option<Uuid>,
    pub title: Option<String>,
    pub content: Vec<HashMap<ContentType, serde_json::Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct CreatePostExtern {
    pub community: String,
    pub parent: Option<Uuid>,
    pub title: Option<String>,
    pub content: Vec<HashMap<ContentType, serde_json::Value>>,
}

#[derive(Serialize, Deserialize)]
pub struct HostQuery {
    host: Option<String>,
}

#[post("/posts/create")]
pub(crate) async fn create_post(
    query: web::Query<HostQuery>,
    pool: web::Data<DBPool>,
    post: web::Json<CreatePost>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, local_user) = authenticate(pool.clone(), request)?;
    match query.host.as_deref() {
        Some(HOSTNAME) => {
            let conn = get_conn_from_pool(pool.clone())?;
            let parent = match post.parent {
                None => None,
                Some(u) => {
                    web::block(move || {
                        use crate::database::actions::post;
                        post::get_post(&conn, &u)
                    })
                    .await?
                }
            };
            let conn = get_conn_from_pool(pool.clone())?;
            let id = post.community.id.clone();
            let community = web::block(move || get_community_by_id(&conn, &id))
                .await?
                .ok_or(RouteError::NotFound)?;

            let new_post = DatabaseNewPost {
                uuid: Uuid::new_v4().to_string(),
                title: post.title.clone(),
                author_id: local_user.user_id,
                created: Utc::now().naive_utc(),
                modified: Utc::now().naive_utc(),
                parent_id: parent.map(|p| p.post.id),
                community_id: community.id,
            };

            let conn = get_conn_from_pool(pool.clone())?;

            let content = post
                .content
                .iter()
                .map(DatabaseContentType::try_from)
                .collect::<Result<Vec<DatabaseContentType>, RouteError>>()?;

            web::block(move || {
                let db_post = put_post(&conn, &new_post)?;
                put_post_contents(&conn, &db_post, &content)
            })
            .await?;
            Ok(HttpResponse::Ok().finish())
        }
        Some(host) => {
            let conn = get_conn_from_pool(pool.clone())?;
            let user = web::block(move || get_name_from_local_user(&conn, local_user)).await?;

            let body = CreatePostExtern {
                community: post.community.id.clone(),
                parent: post.parent,
                title: post.title.clone(),
                content: post.content.clone(),
            };

            let req = make_federated_request(
                awc::Client::post,
                host.to_string(),
                "/fed/posts".to_string(),
                body,
                Some(user.username),
                Option::<()>::None,
            )?
            .await
            .map_err(|_| RouteError::ActixInternal)?;

            if req.status().is_success() {
                Ok(HttpResponse::Ok().finish())
            } else {
                Ok(HttpResponse::InternalServerError().finish())
            }
        }
        None => Ok(HttpResponse::InternalServerError().finish()),
    }
}

#[patch("/posts/{id}")]
pub(crate) async fn edit_post(
    query: web::Query<HostQuery>,
    pool: web::Data<DBPool>,
    web::Path(id): web::Path<Uuid>,
    edit_post: web::Json<EditPost>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, local_user) = authenticate(pool.clone(), request)?;
    match query.host.as_deref() {
        Some(HOSTNAME) => {
            let conn = get_conn_from_pool(pool.clone())?;
            let post = web::block(move || {
                use crate::database::actions::post;
                post::get_post(&conn, &id)
            })
            .await?
            .ok_or(RouteError::NotFound)?;

            // Check permissions
            if !local_user_has_modify_post_permission(pool.clone(), local_user, &post).await? {
                return Ok(HttpResponse::Unauthorized().finish());
            };

            let conn = get_conn_from_pool(pool.clone())?;
            web::block(move || {
                conn.transaction(|| {
                    match &edit_post.title {
                        None => {}
                        Some(n) => {
                            modify_post_title(&conn, post.post.clone(), n)?;
                        }
                    };
                    match &edit_post.content {
                        None => {}
                        Some(n) => {
                            remove_post_contents(&conn, &post.post.clone())?;

                            let content = n
                                .iter()
                                .map(DatabaseContentType::try_from)
                                .collect::<Result<Vec<DatabaseContentType>, RouteError>>()?;

                            put_post_contents(&conn, &post.post, &content)?;
                        }
                    }
                    touch_post(&conn, post.post)?;
                    Ok::<(), RouteError>(())
                })
            })
            .await?;

            Ok(HttpResponse::Ok().finish())
        }
        Some(host) => {
            let conn = get_conn_from_pool(pool.clone())?;
            let user = web::block(move || get_name_from_local_user(&conn, local_user)).await?;

            let req = make_federated_request(
                awc::Client::put,
                host.to_string(),
                format!("/fed/posts/{}", id.to_string()),
                edit_post.into_inner(),
                Some(user.username),
                Option::<()>::None,
            )?
            .await
            .map_err(|_| RouteError::ActixInternal)?;

            if req.status().is_success() {
                Ok(HttpResponse::Ok().finish())
            } else {
                Ok(HttpResponse::InternalServerError().finish())
            }
        }
        None => Ok(HttpResponse::InternalServerError().finish()),
    }
}

async fn local_user_has_modify_post_permission(
    pool: web::Data<DBPool>,
    local_user: DatabaseLocalUser,
    post: &PostInformation,
) -> std::result::Result<bool, actix_web::Error> {
    if local_user.user_id != post.user.id {
        // Check if admin
        let conn = get_conn_from_pool(pool.clone())?;
        let post_to_check = post.clone();
        let admins =
            web::block(move || get_community_admins(&conn, &post_to_check.community)).await?;

        if !admins.into_iter().any(|(u, _)| u.id == local_user.user_id) {
            return Ok(false);
        }
    }

    return Ok(true);
}

#[delete("/posts/{id}")]
pub(crate) async fn delete_post(
    query: web::Query<HostQuery>,
    pool: web::Data<DBPool>,
    web::Path(id): web::Path<Uuid>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, local_user) = authenticate(pool.clone(), request)?;
    match query.host.as_deref() {
        Some(HOSTNAME) => {
            // Get the post first
            let conn = get_conn_from_pool(pool.clone())?;
            let post = web::block(move || {
                use crate::database::actions::post;
                post::get_post(&conn, &id)
            })
            .await?
            .ok_or(RouteError::NotFound)?;

            // Check permissions
            if !local_user_has_modify_post_permission(pool.clone(), local_user, &post).await? {
                return Ok(HttpResponse::Unauthorized().finish());
            };

            let conn = get_conn_from_pool(pool.clone())?;
            web::block(move || remove_post(&conn, post.post)).await?;

            Ok(HttpResponse::Ok().finish())
        }
        Some(host) => {
            let conn = get_conn_from_pool(pool.clone())?;
            let user = web::block(move || get_name_from_local_user(&conn, local_user)).await?;

            let req = make_federated_request(
                awc::Client::delete,
                host.to_string(),
                format!("/fed/posts/{}", id.to_string()),
                "{}".to_string(),
                Some(user.username),
                Option::<()>::None,
            )?
            .await
            .map_err(|_| RouteError::ActixInternal)?;

            if req.status().is_success() {
                Ok(HttpResponse::Ok().finish())
            } else {
                Ok(HttpResponse::InternalServerError().finish())
            }
        }
        None => Ok(HttpResponse::InternalServerError().finish()),
    }
}
