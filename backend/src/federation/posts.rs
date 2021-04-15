//! Federated API endpoints for actions concerning posts
use crate::database::actions::communities::get_community_by_id;
use crate::database::actions::post::{
    get_all_posts, get_all_top_level_posts, get_children_posts_of, get_post, modify_post_title,
    put_post, put_post_contents, remove_post, remove_post_contents, touch_post,
};
use crate::database::get_conn_from_pool;
use crate::database::models::{DatabaseNewPost, DatabasePost};
use crate::federation::schemas::{ContentType, DatabaseContentType, NewPost, Post, User, EditPost};
use crate::internal::authentication::verify_federated_request;
use crate::internal::posts::cache_federated_user;
use crate::util::route_error::RouteError;
use crate::util::{UserDetail, HOSTNAME};
use crate::DBPool;
use actix_web::{delete, get, post, put, web, HttpRequest};
use actix_web::{HttpResponse, Result};
use chrono::{NaiveDateTime, Utc};
use diesel::Connection;
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
use uuid::Uuid;

/// Returns true, always. Used as a default deserialization function.
const fn true_func() -> bool {
    true
}

/// Struct representing the multiple filters that can be set when obtaining posts
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostFilters {
    /// Optional limit of posts to be sent
    limit: Option<u64>,
    /// Optional community to be specified
    community: Option<String>,
    /// Optional minimum date for posts
    min_date: Option<NaiveDateTime>,
    /// Optional author of posts
    author: Option<String>,
    /// Optional hostname of posts
    host: Option<String>,
    /// Optional parent post of posts
    parent_post: Option<Uuid>,
    /// Optional boolean to include comments when getting posts (default: true)
    #[serde(default = "true_func")]
    include_sub_children_posts: bool,
    /// Optional content type of posts
    content_type: Option<ContentType>,
}

/// Federated endpoint to obtain all the locally stored posts matching optionally set filters
#[get("")]
pub(crate) async fn post_matching_filters(
    pool: web::Data<DBPool>,
    req: HttpRequest,
    payload: web::Payload,
    parameters: web::Query<PostFilters>,
) -> Result<HttpResponse> {
    verify_federated_request(req, payload).await?;

    let conn = get_conn_from_pool(pool.clone())?;
    let inc = parameters.include_sub_children_posts;
    let mut posts = {
        let mut p = web::block(move || {
            let posts = if inc {
                get_all_posts(&conn)?
            } else {
                get_all_top_level_posts(&conn)?
            }
            .unwrap_or_default()
            .into_iter()
            .filter(|p| !p.deleted)
            .map(|p| {
                Ok::<_, RouteError>((
                    get_post(&conn, &p.uuid.parse()?)?
                        .ok_or(RouteError::Diesel(diesel::NotFound))?,
                    get_children_posts_of(&conn, &p)?,
                ))
            })
            .collect::<Result<Vec<_>, _>>()?;

            Ok::<_, RouteError>(posts)
        })
        .await?;

        // Sort by descending time
        p.sort_by(|(a, _), (b, _)| b.post.modified.cmp(&a.post.modified));
        p
    };

    if let Some(n) = &parameters.limit {
        posts = posts.into_iter().take(*n as usize).collect();
    }

    if let Some(community_id) = &parameters.community {
        posts = posts
            .into_iter()
            .filter(|(p, _)| &p.community.name == community_id)
            .collect();
    }

    if let Some(date) = &parameters.min_date {
        posts = posts
            .into_iter()
            .filter(|(p, _)| &p.post.created >= date)
            .collect();
    }

    if let Some(author) = &parameters.author {
        posts = posts
            .into_iter()
            .filter(|(p, _)| &p.user.username == author)
            .collect();
    }

    if let Some(host) = &parameters.host {
        posts = posts
            .into_iter()
            .filter(|(p, _)| match &p.user_details {
                UserDetail::Local(_) => host == HOSTNAME,
                UserDetail::Federated(f) => host == &f.host,
            })
            .collect();
    }

    if let Some(ct) = &parameters.content_type {
        posts = posts
            .into_iter()
            .filter(|(p, _)| p.content.iter().any(|c| c.contains_key(ct)))
            .collect();
    }

    let posts = posts
        .into_iter()
        .map(|x| x.try_into())
        .collect::<Result<Vec<Post>, _>>()?;

    // Return type: Vec<Post>
    Ok(HttpResponse::Ok().json(posts))
}

/// Federated endpoint to create a new post on our instance
#[post("")]
pub(crate) async fn new_post_federated(
    pool: web::Data<DBPool>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    let client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?
        .clone();

    let user_id = req
        .headers()
        .get("User-ID")
        .ok_or(RouteError::MissingUserId)?
        .clone();

    let new_post: NewPost = serde_json::from_slice(&verify_federated_request(req, payload).await?)?;

    let conn = get_conn_from_pool(pool.clone())?;

    let content = new_post
        .content
        .iter()
        .map(DatabaseContentType::try_from)
        .collect::<Result<Vec<DatabaseContentType>, RouteError>>()?;

    let post = web::block(move || {
        let user = User {
            id: user_id.to_str()?.to_string(),
            host: client_host.to_str()?.to_string(),
        };

        let post = conn.transaction(|| {
            use crate::database::actions::user;
            cache_federated_user(&conn, &user)?;
            let user = user::get_user_detail_by_name(&conn, &user.id)?;
            let parent = if let Some(u) = &new_post.parent_post {
                Some(get_post(&conn, u)?.ok_or(diesel::NotFound)?)
            } else {
                None
            };

            let community =
                get_community_by_id(&conn, &new_post.community)?.ok_or(diesel::NotFound)?;

            let db_new_post = DatabaseNewPost {
                uuid: Uuid::new_v4().to_string(),
                title: new_post.title.clone(),
                author_id: user.id,
                created: Utc::now().naive_utc(),
                modified: Utc::now().naive_utc(),
                parent_id: parent.map(|p| p.post.id),
                community_id: community.id,
            };

            let post = put_post(&conn, &db_new_post)?;

            put_post_contents(&conn, &post, &content)?;

            Ok::<DatabasePost, diesel::result::Error>(post)
        })?;

        let post = get_post(&conn, &post.uuid.parse()?)?.ok_or(diesel::NotFound)?;

        Post::try_from((post, None))
    })
    .await?;
    Ok(HttpResponse::Ok().json(post))
}

/// Federated endpoint to obtain a post given its UUID
#[get("/{id}")]
pub(crate) async fn get_post_by_id(
    web::Path(id): web::Path<Uuid>,
    pool: web::Data<DBPool>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    verify_federated_request(req, payload).await?;

    let conn = get_conn_from_pool(pool.clone())?;

    let post = web::block(move || get_post(&conn, &id))
        .await?
        .ok_or(RouteError::NotFound)?;

    // Internal post deletion semantic should be opaque to federated requests
    if post.post.deleted {
        return Ok(HttpResponse::NotFound().finish());
    }

    let conn = get_conn_from_pool(pool.clone())?;

    let parent = post.post.clone();
    let children = web::block(move || get_children_posts_of(&conn, &parent)).await?;

    Ok(HttpResponse::Created().json(Post::try_from((post, children))?))
}

/// Federated endpoint to edit a post given its UUID
#[put("/{id}")]
pub(crate) async fn edit_post(
    pool: web::Data<DBPool>,
    web::Path(id): web::Path<Uuid>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    let edit_post: EditPost =
        serde_json::from_slice(&verify_federated_request(req, payload).await?)?;
    // TODO: Check permissions

    let conn = get_conn_from_pool(pool)?;
    web::block(move || {
        // Start new transaction
        conn.transaction(|| {
            // Find the post
            let post = get_post(&conn, &id)?.ok_or(diesel::NotFound)?.post;

            // Internal post deletion semantic should be opaque to federated requests
            if post.deleted {
                return Err(diesel::NotFound.into());
            }

            match &edit_post.title {
                None => {}
                Some(n) => {
                    modify_post_title(&conn, post.clone(), n)?;
                }
            };

            match &edit_post.content {
                None => {}
                Some(n) => {
                    // Now clear everything that existed
                    remove_post_contents(&conn, &post)?;

                    let content = n
                        .iter()
                        .map(DatabaseContentType::try_from)
                        .collect::<Result<Vec<DatabaseContentType>, RouteError>>()?;

                    // Then put the new contents in.
                    put_post_contents(&conn, &post, &content)?;
                }
            }
            touch_post(&conn, post)?;
            Ok::<(), RouteError>(())
        })
    })
    .await?;

    // Nothing to return
    Ok(HttpResponse::Ok().finish())
}

/// Federated endpoint to delete a post given its UUID
#[delete("/{id}")]
pub(crate) async fn delete_post(
    pool: web::Data<DBPool>,
    web::Path(id): web::Path<Uuid>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    verify_federated_request(req, payload).await?;

    let conn = get_conn_from_pool(pool)?;
    web::block(move || {
        conn.transaction(|| {
            let post = get_post(&conn, &id)?.ok_or(diesel::NotFound)?.post;

            // Internal post deletion semantic should be opaque to federated requests
            if post.deleted {
                return Err(diesel::NotFound);
            }

            remove_post(&conn, post)?;

            Ok::<(), diesel::result::Error>(())
        })
    })
    .await?;

    // Nothing to return
    Ok(HttpResponse::Ok().finish())
}
