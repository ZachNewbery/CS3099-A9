use actix_web::{delete, get, post, put, web, HttpRequest};
use actix_web::{HttpResponse, Result};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::database::actions::post::{get_children_posts_of, get_post, clear_post_contents, put_post_contents, modify_post_title, remove_post};
use crate::database::get_conn_from_pool;
use crate::database::models::{DatabaseFederatedUser, DatabaseLocalUser};
use crate::federation::schemas::{ContentType, NewPost, Post, User};
use crate::util::route_error::RouteError;
use crate::DBPool;
use either::Either;
use diesel::Connection;

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
    content_type: Option<String>, // TODO: Enumerate this
}

#[get("/")]
pub(crate) async fn post_matching_filters(
    _pool: web::Data<DBPool>,
    req: HttpRequest,
    _parameters: web::Query<PostFilters>,
) -> Result<HttpResponse> {
    let client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?;
    // TODO: Parse the client host

    let user_id = req
        .headers()
        .get("User-ID")
        .ok_or(RouteError::MissingUserID)?;
    // TODO: Parse the user id

    // TODO: Implement /fed/posts (GET)
    // Return type: Vec<Post>
    Ok(HttpResponse::NotImplemented().finish())
}

#[post("/")]
pub(crate) async fn new_post_federated(
    pool: web::Data<DBPool>,
    req: HttpRequest,
    new_post: web::Json<NewPost>,
) -> Result<HttpResponse> {
    let client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?;
    // TODO: Parse the client host

    // TODO: Check /fed/posts (POST)
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
    web::Path(_id): web::Path<Uuid>,
    pool: web::Data<DBPool>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?
        .to_str()
        .map_err(|e| RouteError::HeaderParse(e))?;
    // TODO: Parse the client host

    let user_id = req
        .headers()
        .get("User-ID")
        .ok_or(RouteError::MissingUserID)?
        .to_str()
        .map_err(|e| RouteError::HeaderParse(e))?;
    // TODO: Parse the user id

    let conn = get_conn_from_pool(pool.clone())?;

    let (post, content, community, user, detail, parent) =
        web::block(move || get_post(&conn, &_id))
            .await?
            .ok_or(RouteError::NotFound)?;

    let conn = get_conn_from_pool(pool.clone())?;

    let parent_ = post.clone();
    let children = web::block(move || get_children_posts_of(&conn, &parent_))
        .await?
        .unwrap_or_default();

    let p = Post {
        id: user
            .username
            .parse()
            .map_err(|e| RouteError::UuidParse(e))?,
        community: community.name,
        parent_post: parent.uuid.parse().map_err(|e| RouteError::UuidParse(e))?,
        children: children
            .into_iter()
            .map(|p| Ok(p.0.uuid.parse()?))
            .collect::<Result<Vec<_>, RouteError>>()?,
        title: community.title,
        content,
        author: User {
            id: user.username,
            host: match detail {
                Either::Left(l) => "REPLACE-ME.com".to_string(),
                Either::Right(f) => f.host,
            },
        },
        modified: post.modified,
        created: post.created,
    };

    Ok(HttpResponse::Created().json(p))

    // Return type: Post
}

#[derive(Clone, Serialize, Deserialize)]
pub struct EditPost {
    title: String,
    content: Vec<ContentType>,
}

#[put("/{id}")]
pub(crate) async fn edit_post(
    pool: web::Data<DBPool>,
    web::Path(id): web::Path<Uuid>,
    edit_post: web::Json<EditPost>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?;
    // TODO: Parse the client host

    let user_id = req
        .headers()
        .get("User-ID")
        .ok_or(RouteError::MissingUserID)?;
    // TODO: Parse the user id

    // TODO: Authenticate user

    // TODO: Implement /fed/posts/id (PUT)
    let conn = get_conn_from_pool(pool)?;
    web::block(move || {
        // Start new transaction
        conn.transaction(|| {
            // Find the post
            let (post, _, _, _, _, _) = get_post(&conn, &id)?
                .ok_or(diesel::NotFound)?;

            let post = modify_post_title(&conn, post, &edit_post.title)?;

            // Now clear everything that existed
            clear_post_contents(&conn, &post)?;

            // Then put the new contents in.
            put_post_contents(&conn, &post, &edit_post.content)?;

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
    let client_host = req
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?;
    // TODO: Parse the client host

    let user_id = req
        .headers()
        .get("User-ID")
        .ok_or(RouteError::MissingUserID)?;
    // TODO: Parse the user id

    // TODO: Authenticate

    // TODO: Implement /fed/posts/id (DEL)
    let conn = get_conn_from_pool(pool)?;
    web::block(move || {
        conn.transaction(|| {
            let (post, _, _, _, _, _) = get_post(&conn, &id)?
                .ok_or(diesel::NotFound)?;

            remove_post(&conn, post)?;

            Ok::<(), diesel::result::Error>(())
        })
    })
        .await?;

    // Nothing to return
    Ok(HttpResponse::Ok().finish())
}
