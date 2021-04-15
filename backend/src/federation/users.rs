//! Federated API endpoints for actions concerning users
use crate::database::actions::local::get_local_user_by_user_id;
use crate::database::actions::post::get_posts_by_user;
use crate::database::actions::post::{get_children_posts_of, get_post};
use crate::database::actions::user::get_local_users;
use crate::database::get_conn_from_pool;
use crate::federation::schemas::Post;
use crate::internal::authentication::verify_federated_request;
use crate::util::route_error::RouteError;
use crate::DBPool;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};

/// Struct representing the body of a sent message
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageParameters {
    /// Title of the message
    title: String,
    /// Content of the message
    content: String,
}

/// Struct representing the query when searching users
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SearchUsersParameters {
    /// Search string
    prefix: Option<String>,
}

/// Federated endpoint to search local users by a specified search prefix
#[get("")]
pub(crate) async fn search_users(
    pool: web::Data<DBPool>,
    web::Query(query): web::Query<SearchUsersParameters>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    verify_federated_request(req, payload).await?;

    let conn = get_conn_from_pool(pool.clone())?;
    let mut users = web::block(move || get_local_users(&conn)).await?;

    if let Some(p) = query.prefix {
        users = users
            .into_iter()
            .filter(|(u, _l)| u.username.starts_with(&p))
            .collect::<Vec<(_, _)>>();
    }

    Ok(HttpResponse::Ok().json(
        users
            .into_iter()
            .map(|(u, _l)| u.username)
            .collect::<Vec<String>>(),
    ))
}

/// Struct representing the required details of a local user
#[derive(Clone, Serialize, Deserialize)]
struct UserDetails {
    /// Username of the local user
    id: String,
    /// Bio of the local user
    about: Option<String>,
    /// Avatar URL of the local user
    avatar_url: Option<String>,
    /// Array of posts made by the local user
    posts: Vec<Post>,
}

/// Federated endpoint to retrieve a local user given their username
#[get("/{id}")]
pub(crate) async fn user_by_id(
    web::Path(id): web::Path<String>,
    pool: web::Data<DBPool>,
    request: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    use std::convert::TryInto;

    verify_federated_request(request, payload).await?;

    let conn = get_conn_from_pool(pool.clone())?;
    let username = id.clone();
    let (user, local) = web::block(move || get_local_user_by_user_id(&conn, &username))
        .await?
        .ok_or(RouteError::NotFound)?;

    let conn = get_conn_from_pool(pool)?;
    let posts = web::block(move || {
        let posts = get_posts_by_user(&conn, &user)?
            .unwrap_or_default()
            .into_iter()
            .map(|p| {
                (
                    get_post(&conn, &p.uuid.parse()?)?
                        .ok_or(RouteError::Diesel(diesel::NotFound))?,
                    get_children_posts_of(&conn, &p)?,
                )
                    .try_into()
            })
            .collect::<Result<Vec<Post>, RouteError>>()?;
        Ok::<_, RouteError>(posts)
    })
    .await?;

    // Return type: { id, posts }
    Ok(HttpResponse::Ok().json(UserDetails {
        id,
        about: local.bio,
        avatar_url: local.avatar,
        posts,
    }))
}

/// Unimplemented federated endpoint to send a user a message
#[post("/{id}")]
pub(crate) async fn send_user_message(
    _parameters: web::Query<MessageParameters>,
    web::Path(_id): web::Path<String>,
    req: HttpRequest,
    payload: web::Payload,
) -> Result<HttpResponse> {
    // TODO: /fed/users/id (POST)
    verify_federated_request(req, payload).await?;
    // No return type
    Ok(HttpResponse::NotImplemented().finish())
}
