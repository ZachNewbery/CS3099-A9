use crate::database::actions::post::{
    get_all_posts, get_all_top_level_posts, get_children_posts_of, get_post, modify_post_title,
    put_post_contents, remove_post, remove_post_contents, touch_post,
};
use crate::database::get_conn_from_pool;
use crate::federation::schemas::{ContentType, NewPost, Post};
use crate::internal::authentication::verify_federated_request;
use crate::util::route_error::RouteError;
use crate::util::{UserDetail, HOSTNAME};
use crate::DBPool;
use actix_web::{delete, get, post, put, web, HttpRequest};
use actix_web::{HttpResponse, Result};
use chrono::NaiveDateTime;
use diesel::Connection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::{TryFrom, TryInto};
use uuid::Uuid;

const fn true_func() -> bool {
    false
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostFilters {
    limit: Option<u64>,
    community: Option<String>,
    min_date: Option<NaiveDateTime>,
    author: Option<String>,
    host: Option<String>,
    parent_post: Option<Uuid>,
    #[serde(default = "true_func")]
    include_sub_children_posts: bool,
    content_type: Option<ContentType>,
}

#[get("")]
pub(crate) async fn post_matching_filters(
    pool: web::Data<DBPool>,
    req: HttpRequest,
    payload: web::Payload,
    parameters: web::Query<PostFilters>,
) -> Result<HttpResponse> {
    let _client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?;

    let _user_id = req
        .headers()
        .get("User-ID")
        .ok_or(RouteError::MissingUserId)?;

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

#[post("")]
pub(crate) async fn new_post_federated(
    _pool: web::Data<DBPool>,
    req: HttpRequest,
    payload: web::Payload,
    _new_post: web::Json<NewPost>,
) -> Result<HttpResponse> {
    // TODO: Authentication for /fed/posts (POST)
    let _client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?;

    verify_federated_request(req, payload).await?;

    // TODO: Implement /fed/posts (POST)
    // let conn = pool
    //     .get()
    //     .map_err(|_| HttpResponse::InternalServerError().finish())?;
    //
    // web::block(move || {
    //     create_federated_post(&conn, new_post.clone())?;
    //     Ok::<(), diesel::result::Error>(())
    // })
    // .await
    // .map_err(|_| HttpResponse::InternalServerError().finish())?;

    // Return type: Post

    Ok(HttpResponse::NotImplemented().finish())
}

#[get("/{id}")]
pub(crate) async fn get_post_by_id(
    web::Path(id): web::Path<Uuid>,
    pool: web::Data<DBPool>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    let _client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?
        .to_str()
        .map_err(RouteError::HeaderParse)?;

    let _user_id = req
        .headers()
        .get("User-ID")
        .ok_or(RouteError::MissingUserId)?
        .to_str()
        .map_err(RouteError::HeaderParse)?;

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

#[derive(Clone, Serialize, Deserialize)]
pub struct EditPost {
    pub title: Option<String>,
    pub content: Option<Vec<HashMap<ContentType, serde_json::Value>>>,
}

#[put("/{id}")]
pub(crate) async fn edit_post(
    pool: web::Data<DBPool>,
    web::Path(id): web::Path<Uuid>,
    edit_post: web::Json<EditPost>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    let _client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?;

    let _user_id = req
        .headers()
        .get("User-ID")
        .ok_or(RouteError::MissingUserId)?;

    verify_federated_request(req, payload).await?;
    // TODO: Check permissions

    let conn = get_conn_from_pool(pool)?;
    web::block(move || {
        // Start new transaction
        conn.transaction(|| {
            // Find the post
            let post = get_post(&conn, &id)?.ok_or(diesel::NotFound)?.post;

            // Internal post deletion semantic should be opaque to federated requests
            if post.deleted {
                return Err(diesel::NotFound);
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

                    // Then put the new contents in.
                    put_post_contents(&conn, &post, &n)?;
                }
            }
            touch_post(&conn, post)?;
            Ok::<(), diesel::result::Error>(())
        })
    })
    .await?;

    // Nothing to return
    Ok(HttpResponse::Ok().finish())
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DeletePost {
    id: Uuid,
}

#[delete("/{id}")]
pub(crate) async fn delete_post(
    pool: web::Data<DBPool>,
    web::Path(id): web::Path<Uuid>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    let _client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?;

    let _user_id = req
        .headers()
        .get("User-ID")
        .ok_or(RouteError::MissingUserId)?;

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
