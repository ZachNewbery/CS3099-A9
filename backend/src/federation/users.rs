use crate::database::actions::local::get_local_user_by_user_id;
use crate::database::actions::post::get_posts_by_user;
use crate::database::actions::post::{get_children_posts_of, get_post};
use crate::database::actions::user::get_local_users;
use crate::database::get_conn_from_pool;
use crate::federation::schemas::Post;
use crate::util::route_error::RouteError;
use crate::DBPool;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MessageParameters {
    title: String,
    content: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SearchUsersParameters {
    prefix: Option<String>,
}

#[get("/")]
pub(crate) async fn search_users(
    pool: web::Data<DBPool>,
    web::Query(query): web::Query<SearchUsersParameters>,
) -> Result<HttpResponse> {
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

#[derive(Clone, Serialize, Deserialize)]
struct UserDetails {
    id: String,
    about: Option<String>,
    avatar_url: Option<String>,
    posts: Vec<Post>,
}

#[get("/{id}")]
pub(crate) async fn user_by_id(
    web::Path(id): web::Path<String>,
    pool: web::Data<DBPool>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    use std::convert::TryInto;
    // TODO: /fed/users/id (GET)
    let _client_host = request
        .headers()
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?
        .to_str()
        .map_err(RouteError::HeaderParse)?;

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
                    get_post(&conn, &p.uuid.parse().map_err(RouteError::UuidParse)?)?
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

#[post("/{id}")]
pub(crate) async fn send_user_message(
    _parameters: web::Query<MessageParameters>,
    web::Path(_id): web::Path<String>,
) -> Result<HttpResponse> {
    // TODO: /fed/users/id (POST)
    // No return type
    Ok(HttpResponse::NotImplemented().finish())
}
