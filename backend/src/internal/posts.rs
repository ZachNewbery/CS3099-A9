use crate::database::actions::communities::get_community;
use crate::database::actions::post::{
    get_all_posts, get_children_posts_of, get_posts_of_community, PostInformation,
};
use crate::database::get_conn_from_pool;
use crate::database::models::{DatabaseFederatedUser, DatabaseLocalUser};
use crate::federation::schemas::{ContentType, UpdatePost, User};
use crate::internal::authentication::authenticate;
use crate::internal::LocatedCommunity;
use crate::util::route_error::RouteError;
use crate::util::HOSTNAME;
use crate::DBPool;
use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse, Result};
use chrono::NaiveDateTime;
use either::Either;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct GetPost {
    host: Option<String>,
    community: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct LocatedPost {
    pub(crate) id: Uuid,
    pub(crate) community: LocatedCommunity,
    pub(crate) parent_post: Option<Uuid>,
    pub(crate) children: Vec<Uuid>,
    pub(crate) title: String,
    pub(crate) content: Vec<ContentType>,
    pub(crate) author: User,
    pub(crate) modified: NaiveDateTime,
    pub(crate) created: NaiveDateTime,
}

#[get("/posts/{id}")]
pub(crate) async fn get_post(
    web::Path(_id): web::Path<Uuid>,
    _query: web::Query<GetPost>,
    pool: web::Data<DBPool>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, _local_user) = authenticate(pool.clone(), request)?;

    // TODO: Add federated lookup
    let conn = get_conn_from_pool(pool.clone())?;
    let post = web::block(move || {
        use crate::database::actions::post;
        post::get_post(&conn, &_id)
    })
    .await?
    .ok_or(HttpResponse::NotFound().finish())?;

    let conn = get_conn_from_pool(pool.clone())?;
    let parent = post.post.clone();
    let children = web::block(move || get_children_posts_of(&conn, &parent))
        .await?
        .unwrap_or_default();

    let lp = LocatedPost {
        id: post
            .post
            .uuid
            .parse()
            .map_err(|e| RouteError::UuidParse(e))?,
        community: LocatedCommunity::Local {
            id: post.community.name,
        },
        parent_post: post
            .parent
            .map(|u| u.uuid.parse().map_err(|e| RouteError::UuidParse(e)))
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
                Either::Left(_) => HOSTNAME.to_string(),
                Either::Right(f) => f.host,
            },
        },
        modified: post.post.modified,
        created: post.post.created,
    };

    // Return type: a monstrosity, honestly.
    Ok(HttpResponse::Ok().json(lp))
}

#[get("/posts")]
pub(crate) async fn list_posts(
    _query: web::Query<GetPost>,
    pool: web::Data<DBPool>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, _local_user) = authenticate(pool.clone(), request)?;

    let conn = get_conn_from_pool(pool.clone())?;
    // Specialised code path for a community being specified
    let posts = web::block(move || {
        let posts = match &_query.community {
            None => get_all_posts(&conn),
            Some(c) => {
                let community = get_community(&conn, c)?.ok_or(diesel::NotFound)?;
                get_posts_of_community(&conn, &community)
            }
        }?
        .unwrap_or_default();
        posts
            .into_iter()
            .map(|p| {
                use crate::database::actions::post;
                let post = post::get_post(
                    &conn,
                    &p.uuid.parse().map_err(|e| RouteError::UuidParse(e))?,
                )?
                .ok_or(diesel::NotFound)?;
                let children = get_children_posts_of(&conn, &post.post)?.unwrap_or_default();
                Ok((post, children))
            })
            .collect::<Result<Vec<(PostInformation, Vec<PostInformation>)>, RouteError>>()
    })
    .await?;

    let posts = posts
        .into_iter()
        .map(|(p, c)| {
            Ok(LocatedPost {
                id: p.post.uuid.parse().map_err(|e| RouteError::UuidParse(e))?,
                community: LocatedCommunity::Local {
                    id: p.community.name,
                },
                parent_post: None,
                children: c
                    .into_iter()
                    .map(|h| Ok(h.post.uuid.parse().map_err(|e| RouteError::UuidParse(e))?))
                    .collect::<Result<Vec<Uuid>, RouteError>>()?,
                title: p.post.title,
                content: p.content,
                author: User {
                    id: p.user.username,
                    host: match p.user_details {
                        Either::Left(_) => HOSTNAME.to_string(),
                        Either::Right(f) => f.host,
                    },
                },
                modified: p.post.modified,
                created: p.post.created,
            })
        })
        .collect::<Result<Vec<LocatedPost>, RouteError>>()?;

    Ok(HttpResponse::Ok().json(posts))
}

#[derive(Serialize, Deserialize)]
pub struct SearchPosts {
    #[serde(flatten)]
    host_community: GetPost,
    search: String,
}

#[get("/posts/search")]
pub(crate) async fn search_posts(
    _query: web::Query<GetPost>,
    pool: web::Data<DBPool>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, _local_user) = authenticate(pool.clone(), request)?;

    // TODO: Implement /internal/posts/search (GET)

    // Return type: Vec<Posts>
    unimplemented!()
}

#[derive(Serialize, Deserialize)]
pub struct CreatePost {
    pub community: LocatedCommunity,
    pub parent: Option<Uuid>,
    pub title: Option<String>,
    pub content: Vec<ContentType>,
}

#[post("/posts/create")]
pub(crate) async fn create_post(
    pool: web::Data<DBPool>,
    _post: web::Data<CreatePost>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, _local_user) = authenticate(pool.clone(), request)?;

    // TODO: Implement /internal/posts/create (POST)

    // Return type: none
    unimplemented!()
}

#[patch("/posts/{id}")]
pub(crate) async fn edit_post(
    pool: web::Data<DBPool>,
    web::Path(_id): web::Path<Uuid>,
    _post: web::Data<UpdatePost>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, _local_user) = authenticate(pool.clone(), request)?;

    // TODO: Implement /internal/posts/id (PATCH)

    // Return type: post with updated values
    unimplemented!()
}

#[delete("/posts/{id}")]
pub(crate) async fn delete_post(
    pool: web::Data<DBPool>,
    web::Path(_id): web::Path<Uuid>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, _local_user) = authenticate(pool.clone(), request)?;

    // TODO: Implement /internal/posts/id (DELETE)

    // Return type: post with updated values
    unimplemented!()
}
