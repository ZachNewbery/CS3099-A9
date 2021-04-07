use crate::database::actions::communities::{get_community, get_community_admins};
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
use crate::federation::schemas::{ContentType, User};
use crate::internal::authentication::{authenticate, make_federated_request};
use crate::internal::{get_known_hosts, LocatedCommunity};
use crate::util::route_error::RouteError;
use crate::DBPool;
use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse, Result};
use chrono::{DateTime, Utc};
use diesel::Connection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
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
    pub(crate) modified: DateTime<Utc>,
    pub(crate) created: DateTime<Utc>,
    #[serde(default)]
    pub(crate) deleted: bool,
}

#[get("/posts/{id}")]
pub(crate) async fn get_post(
    web::Path(id): web::Path<Uuid>,
    _query: web::Query<GetPost>,
    pool: web::Data<DBPool>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, local_user) = authenticate(pool.clone(), request)?;
    let conn = get_conn_from_pool(pool.clone())?;
    let post = web::block(move || {
        use crate::database::actions::post;
        post::get_post(&conn, &id)
    })
    .await?;

    let conn = get_conn_from_pool(pool.clone())?;

    let lp = match post {
        None => {
            let u = web::block(move || get_name_from_local_user(&conn, local_user)).await?;
            get_post_extern(id, pool, u.username).await?
        }
        Some(p) => get_post_local(pool, p).await?,
    };

    // Return type: a monstrosity, honestly.
    Ok(HttpResponse::Ok().json(lp))
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
        title: post.post.title,
        content: post.content,
        author: (post.user, post.user_details).into(),
        modified: DateTime::<Utc>::from_utc(post.post.modified, Utc),
        created: DateTime::<Utc>::from_utc(post.post.created, Utc),
        deleted: post.post.deleted,
    };

    Ok(lp)
}

pub(crate) async fn get_post_extern(
    uuid: Uuid,
    pool: web::Data<DBPool>,
    username: String,
) -> Result<LocatedPost, RouteError> {
    let mut post: Option<LocatedPost> = None;
    let mut q_string = "/fed/posts/".to_owned();
    q_string.push_str(&uuid.to_string());
    for host in get_known_hosts().iter() {
        let mut query = make_federated_request(
            awc::Client::get,
            host.to_string(),
            q_string.clone(),
            "{}".to_string(),
            Some(username.clone()),
            Option::<()>::None,
        )?
        .await
        .map_err(|_| RouteError::ActixInternal)?;

        if query.status().is_success() {
            let body = query.body().await?;

            let s_post: String =
                String::from_utf8(body.to_vec()).map_err(|_| RouteError::ActixInternal)?;

            post = serde_json::from_str(&s_post)?;
            break;
        }
    }

    if let Some(p) = post {
        // add author to federated users for caching :)
        let author = p.author.clone();
        let conn = get_conn_from_pool(pool.clone()).map_err(|_| RouteError::ActixInternal)?;
        if !get_user_detail_by_name(&conn, &author.id).is_ok() {
            let _ = insert_new_federated_user(&conn, author);
        }
        Ok(p)
    } else {
        Err(RouteError::NotFound)
    }
}

#[get("/posts")]
pub(crate) async fn list_posts(
    query: web::Query<GetPost>,
    pool: web::Data<DBPool>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, _local_user) = authenticate(pool.clone(), request)?;
    // Specialised code path for a community being specified
    // control flow
    // parse host =>
    // HOSTNAME => list local posts with community
    // other string => query external host with hostname
    // none => pass to next function to work with community
    // parse community
    // Some(c) => check if local or remote community
    // local can pass to list_local_posts
    // remote need to find hostname!!!!!
    // none => concat ALL posts

    // this is only local stuff!
    let posts = list_local_posts(query.community.clone(), pool).await?;

    Ok(HttpResponse::Ok().json(posts))
}

pub(crate) async fn list_local_posts(
    community: Option<String>,
    pool: web::Data<DBPool>,
) -> Result<Vec<LocatedPost>, RouteError> {
    let conn = get_conn_from_pool(pool.clone()).map_err(|_| RouteError::ActixInternal)?;
    let posts = web::block(move || {
        let posts = match &community {
            None => get_all_top_level_posts(&conn),
            Some(c) => {
                let community = get_community(&conn, c)?.ok_or(diesel::NotFound)?;

                get_top_level_posts_of_community(&conn, &community)
            }
        }?
        .unwrap_or_default();

        posts
            .into_iter()
            .map(|p| {
                use crate::database::actions::post;
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
                title: p.post.title,
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

#[derive(Serialize, Deserialize, Clone)]
pub struct SearchPosts {
    #[serde(flatten)]
    host_community: GetPost,
    search: String,
}

#[get("/posts/search")]
pub(crate) async fn search_posts(
    query: web::Query<SearchPosts>,
    pool: web::Data<DBPool>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, _local_user) = authenticate(pool.clone(), request)?;

    // TODO: Can we not copy and paste things? Apparently not. I swear this works.

    // Specialised code path for a community being specified
    let conn = get_conn_from_pool(pool.clone())?;
    let query2 = query.clone();
    let posts = web::block(move || {
        let posts = match &query2.host_community.community {
            None => get_all_top_level_posts(&conn),
            Some(c) => {
                let community = get_community(&conn, c)?.ok_or(diesel::NotFound)?;
                get_top_level_posts_of_community(&conn, &community)
            }
        }?
        .unwrap_or_default();
        posts
            .into_iter()
            .map(|p| {
                use crate::database::actions::post;
                let post = post::get_post(&conn, &p.uuid.parse()?)?.ok_or(diesel::NotFound)?;
                let children = get_children_posts_of(&conn, &post.post)?.unwrap_or_default();
                Ok((post, children))
            })
            .collect::<Result<Vec<(PostInformation, Vec<PostInformation>)>, RouteError>>()
    })
    .await?;

    let posts = posts
        .into_iter()
        .filter(|(p, _)| {
            p.content.iter().any(|c| {
                let content = match c {
                    ContentType::Text { text } => text,
                    ContentType::Markdown { markdown: text } => text,
                };
                content.contains(&query.search)
            })
        })
        .map(|(p, c)| {
            Ok(LocatedPost {
                id: p.post.uuid.parse()?,
                community: LocatedCommunity::Local {
                    id: p.community.name,
                },
                parent_post: None,
                children: c
                    .into_iter()
                    .map(|h| Ok(h.post.uuid.parse()?))
                    .collect::<Result<Vec<_>, RouteError>>()?,
                title: p.post.title,
                content: p.content,
                author: (p.user, p.user_details).into(),
                modified: DateTime::<Utc>::from_utc(p.post.modified, Utc),
                created: DateTime::<Utc>::from_utc(p.post.created, Utc),
                deleted: p.post.deleted,
            })
        })
        .collect::<Result<Vec<LocatedPost>, RouteError>>()?;

    Ok(HttpResponse::Ok().json(posts))
}

#[derive(Serialize, Deserialize)]
pub struct CreatePost {
    pub community: LocatedCommunity,
    pub parent: Option<Uuid>,
    pub title: String,
    pub content: Vec<ContentType>,
}

#[post("/posts/create")]
pub(crate) async fn create_post(
    pool: web::Data<DBPool>,
    post: web::Json<CreatePost>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, _local_user) = authenticate(pool.clone(), request)?;

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

    match &post.community {
        LocatedCommunity::Local { id } => {
            let conn = get_conn_from_pool(pool.clone())?;
            let id = id.clone();
            let community = web::block(move || get_community(&conn, &id))
                .await?
                .ok_or(RouteError::NotFound)?;

            let new_post = DatabaseNewPost {
                uuid: Uuid::new_v4().to_string(),
                title: post.title.clone(),
                author_id: _local_user.id,
                created: Utc::now().naive_utc(),
                modified: Utc::now().naive_utc(),
                parent_id: parent.map(|p| p.post.id),
                community_id: community.id,
            };

            let conn = get_conn_from_pool(pool.clone())?;
            web::block(move || {
                let db_post = put_post(&conn, &new_post)?;
                put_post_contents(&conn, &db_post, &post.content[..])
            })
            .await?;
            Ok(HttpResponse::Ok().finish())
        }
        LocatedCommunity::Federated { .. } => Ok(HttpResponse::NotImplemented().finish()),
    }
}

#[patch("/posts/{id}")]
pub(crate) async fn edit_post(
    pool: web::Data<DBPool>,
    web::Path(id): web::Path<Uuid>,
    edit_post: web::Json<EditPost>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, _local_user) = authenticate(pool.clone(), request)?;

    // Get the post first
    let conn = get_conn_from_pool(pool.clone())?;
    let post = web::block(move || {
        use crate::database::actions::post;
        post::get_post(&conn, &id)
    })
    .await?
    .ok_or(RouteError::NotFound)?; // change here to send to federated host?

    // Check permissions
    if !local_user_has_modify_post_permission(pool.clone(), _local_user, &post).await? {
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
                    put_post_contents(&conn, &post.post, n)?;
                }
            }
            touch_post(&conn, post.post)?;
            Ok::<(), diesel::result::Error>(())
        })
    })
    .await?;

    Ok(HttpResponse::Ok().finish())
}

async fn local_user_has_modify_post_permission(
    pool: web::Data<DBPool>,
    local_user: DatabaseLocalUser,
    post: &PostInformation,
) -> std::result::Result<bool, actix_web::Error> {
    if local_user.id != post.user.id {
        // Check if admin
        let conn = get_conn_from_pool(pool.clone())?;
        let post_to_check = post.clone();
        let admins =
            web::block(move || get_community_admins(&conn, &post_to_check.community)).await?;

        if !admins.into_iter().any(|(u, _)| u.id == local_user.id) {
            return Ok(false);
        }
    }

    return Ok(true);
}

#[delete("/posts/{id}")]
pub(crate) async fn delete_post(
    pool: web::Data<DBPool>,
    web::Path(id): web::Path<Uuid>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, _local_user) = authenticate(pool.clone(), request)?;

    // Get the post first
    let conn = get_conn_from_pool(pool.clone())?;
    let post = web::block(move || {
        use crate::database::actions::post;
        post::get_post(&conn, &id)
    })
    .await?
    .ok_or(RouteError::NotFound)?; // change here for federation

    // Check permissions
    if !local_user_has_modify_post_permission(pool.clone(), _local_user, &post).await? {
        return Ok(HttpResponse::Unauthorized().finish());
    };

    let conn = get_conn_from_pool(pool.clone())?;
    web::block(move || remove_post(&conn, post.post)).await?;

    Ok(HttpResponse::Ok().finish())
}
