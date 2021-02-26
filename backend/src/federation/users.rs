use crate::database::actions::user::get_local_users;
use crate::database::get_conn_from_pool;
use crate::DBPool;
use actix_web::{get, post, web, HttpResponse, Result};
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

#[get("/{id}")]
pub(crate) async fn user_by_id(web::Path(_id): web::Path<String>) -> Result<HttpResponse> {
    // TODO: /fed/users/id (GET)
    // Return type: { id, posts }
    Ok(HttpResponse::NotImplemented().finish())
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
