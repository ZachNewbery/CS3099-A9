use actix_web::{delete, get, post, put, web, HttpRequest};
use actix_web::{HttpResponse, Result};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::actions::post::{
    get_children_posts_of, get_post, modify_post_title, put_post_contents, remove_post,
    remove_post_contents,
};
use crate::database::get_conn_from_pool;

use crate::federation::schemas::{ContentType, NewPost, Post, User};
use crate::util::route_error::RouteError;
use crate::util::HOSTNAME;
use crate::DBPool;
use diesel::Connection;
use either::Either;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostFilters {
    limit: Option<u64>,
    community: Option<String>,
    min_date: Option<NaiveDateTime>,
    author: Option<String>,
    host: Option<String>,
    parent_post: Option<Uuid>,
    include_sub_children_posts: Option<bool>,
    content_type: Option<ContentType>,
}

#[get("/")]
pub(crate) async fn post_matching_filters(
    _pool: web::Data<DBPool>,
    req: HttpRequest,
    _parameters: web::Query<PostFilters>,
) -> Result<HttpResponse> {
    // TODO: Authentication for /fed/posts (filter) (GET)
    let _client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?;

    let _user_id = req
        .headers()
        .get("User-ID")
        .ok_or(RouteError::MissingUserID)?;

    // TODO: Implement /fed/posts (filter) (GET)
    // Return type: Vec<Post>
    Ok(HttpResponse::NotImplemented().finish())
}

#[post("/")]
pub(crate) async fn new_post_federated(
    _pool: web::Data<DBPool>,
    req: HttpRequest,
    _new_post: web::Json<NewPost>,
) -> Result<HttpResponse> {
    // TODO: Authentication for /fed/posts (POST)
    let _client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?;

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
) -> Result<HttpResponse> {
    // TODO: Authentication for /fed/posts (GET)
    let _client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?
        .to_str()
        .map_err(RouteError::HeaderParse)?;

    let _user_id = req
        .headers()
        .get("User-ID")
        .ok_or(RouteError::MissingUserID)?
        .to_str()
        .map_err(RouteError::HeaderParse)?;

    let conn = get_conn_from_pool(pool.clone())?;

    let post = web::block(move || get_post(&conn, &id))
        .await?
        .ok_or(RouteError::NotFound)?;

    let conn = get_conn_from_pool(pool.clone())?;

    let parent = post.post.clone();
    let children = web::block(move || get_children_posts_of(&conn, &parent))
        .await?
        .unwrap_or_default();

    let p = Post {
        id: post.user.username.parse().map_err(RouteError::UuidParse)?,
        community: post.community.name,
        parent_post: post
            .parent
            .map(|u| u.uuid.parse().map_err(RouteError::UuidParse))
            .transpose()?,
        children: children
            .into_iter()
            .map(|p| Ok(p.post.uuid.parse()?))
            .collect::<Result<Vec<_>, RouteError>>()?,
        title: post.post.title,
        content: post.content,
        author: User {
            id: post.user.username,
            host: match post.user_details {
                Either::Left(_l) => HOSTNAME.to_string(),
                Either::Right(f) => f.host,
            },
        },
        modified: post.post.modified,
        created: post.post.created,
    };

    Ok(HttpResponse::Created().json(p))
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EditPost {
    pub title: Option<String>,
    pub content: Option<Vec<ContentType>>,
}

#[put("/{id}")]
pub(crate) async fn edit_post(
    pool: web::Data<DBPool>,
    web::Path(id): web::Path<Uuid>,
    edit_post: web::Json<EditPost>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    // TODO: Authentication for /fed/posts (PUT)
    let _client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?;

    let _user_id = req
        .headers()
        .get("User-ID")
        .ok_or(RouteError::MissingUserID)?;

    let conn = get_conn_from_pool(pool)?;
    web::block(move || {
        // Start new transaction
        conn.transaction(|| {
            // Find the post
            let post = get_post(&conn, &id)?.ok_or(diesel::NotFound)?.post;

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
) -> Result<HttpResponse> {
    // TODO: Authentication for /fed/posts (DELETE)
    let _client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?;

    let _user_id = req
        .headers()
        .get("User-ID")
        .ok_or(RouteError::MissingUserID)?;

    let conn = get_conn_from_pool(pool)?;
    web::block(move || {
        conn.transaction(|| {
            let post = get_post(&conn, &id)?.ok_or(diesel::NotFound)?.post;

            remove_post(&conn, post)?;

            Ok::<(), diesel::result::Error>(())
        })
    })
    .await?;

    // Nothing to return
    Ok(HttpResponse::Ok().finish())
}
