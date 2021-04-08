use crate::database::actions::local::{
    get_local_user_by_credentials, get_local_user_by_username_email, insert_new_local_user,
    update_local_user, update_session,
};
use crate::database::actions::post::{get_children_posts_of, get_post, get_posts_by_user};
use crate::database::actions::user::{get_user_detail, get_user_detail_by_name};
use crate::database::get_conn_from_pool;
use crate::database::models::DatabaseFederatedUser;
use crate::federation::schemas::Post;
use crate::internal::authentication::{
    authenticate, generate_session, make_federated_request, Token,
};
use crate::util::route_error::RouteError;
use crate::util::{UserDetail, HOSTNAME};
use crate::{database, DBPool};
use actix_web::{get, post, put, HttpResponse};
use actix_web::{web, HttpRequest};
use diesel::Connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewLocalUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[post("/new_user")]
pub(crate) async fn new_user(
    pool: web::Data<DBPool>,
    new_user: web::Json<NewLocalUser>,
) -> actix_web::Result<HttpResponse> {
    let conn = pool
        .get()
        .map_err(|_| HttpResponse::ServiceUnavailable().finish())?;

    web::block(move || {
        // Check email and username against database
        if get_local_user_by_username_email(&conn, &new_user.username, &new_user.email)?.is_none() {
            // Insert new record into database
            insert_new_local_user(&conn, new_user.clone())?;
        }

        Ok::<(), diesel::result::Error>(())
    })
    .await?;

    Ok(HttpResponse::Ok().finish())
}

// We will use email + password for this
#[derive(Serialize, Deserialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginOutput {
    pub username: String,
    pub host: String,
    pub avatar: Option<String>,
    pub bio: Option<String>,
    #[serde(flatten)]
    pub new_token: NewToken,
}

#[post("/login")]
pub(crate) async fn login(
    pool: web::Data<DBPool>,
    login_info: web::Json<Login>,
) -> actix_web::Result<HttpResponse> {
    let conn = database::get_conn_from_pool(pool.clone())?;

    // Check credentials against database
    let (user, local_user) = web::block(move || {
        get_local_user_by_credentials(&conn, &login_info.email, &login_info.password)
    })
    .await
    .map_err(|_| HttpResponse::InternalServerError().finish())?
    .ok_or_else(|| HttpResponse::Unauthorized().finish())?; // User not found

    let new_session = generate_session();

    // Generate JWT Token
    let token = Token::new(local_user.id, &new_session)
        .generate_token()
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    let conn = database::get_conn_from_pool(pool)?;

    let local_user_copy = local_user.clone();
    // Invalidate the old session
    web::block(move || update_session(&conn, &local_user, &new_session))
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().json(LoginOutput {
        username: user.username,
        host: HOSTNAME.to_string(),
        avatar: local_user_copy.avatar,
        bio: local_user_copy.bio,
        new_token: NewToken {
            token,
            token_type: String::from("bearer"),
        },
    }))
}

#[post("/logout")]
pub(crate) async fn logout(
    request: HttpRequest,
    pool: web::Data<DBPool>,
) -> actix_web::Result<HttpResponse> {
    // Verify token validity
    let (_, local_user) = authenticate(pool.clone(), request)?;

    // Invalidate token by blanking out session
    let conn = get_conn_from_pool(pool)?;
    web::block(move || update_session(&conn, &local_user, ""))
        .await
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    Ok(HttpResponse::Ok().finish())
}

#[derive(Serialize, Deserialize)]
pub struct EditProfile {
    pub avatar: Option<String>,
    pub bio: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NewToken {
    pub token: String,
    pub token_type: String,
}

#[put("/edit_profile")]
pub(crate) async fn edit_profile(
    request: HttpRequest,
    edit_profile: web::Json<EditProfile>,
    pool: web::Data<DBPool>,
) -> actix_web::Result<HttpResponse> {
    // Verify token validity
    let (_, local_user) = authenticate(pool.clone(), request)?;

    let conn = get_conn_from_pool(pool.clone())?;

    let new_session = generate_session();
    let token = Token::new(local_user.id, &new_session)
        .generate_token()
        .map_err(|_| HttpResponse::InternalServerError().finish())?;

    web::block(move || {
        conn.transaction(|| {
            let u = update_local_user(&conn, local_user, &*edit_profile)?;
            update_session(&conn, &u, &new_session)
        })
    })
    .await?;

    // Return type: new token
    Ok(HttpResponse::Ok().json(NewToken {
        token,
        token_type: String::from("bearer"),
    }))
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UserProfile {
    pub id: String,
    pub about: Option<String>,
    pub avatar_url: Option<String>,
}

#[get("/user/{name}")]
pub(crate) async fn get_user(
    request: HttpRequest,
    web::Path(name): web::Path<String>,
    pool: web::Data<DBPool>,
) -> actix_web::Result<HttpResponse> {
    use std::convert::TryInto;
    let (_, _) = authenticate(pool.clone(), request)?;

    let conn = get_conn_from_pool(pool.clone())?;
    let user = web::block(move || get_user_detail_by_name(&conn, &name)).await?;
    let uname = user.clone().username;

    let conn = get_conn_from_pool(pool.clone())?;
    let user_copy = user.clone();
    let user_details = web::block(move || get_user_detail(&conn, &user_copy)).await?;

    let conn = get_conn_from_pool(pool)?;
    let _posts = web::block(move || {
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

    let profile = match user_details {
        UserDetail::Local(l) => UserProfile {
            id: uname,
            about: l.bio,
            avatar_url: l.avatar,
        },
        UserDetail::Federated(f) => get_extern_user(f, uname).await?,
    };

    Ok(HttpResponse::Ok().json(profile))
}

pub(crate) async fn get_extern_user(
    user: DatabaseFederatedUser,
    name: String,
) -> Result<UserProfile, RouteError> {
    let mut q_string = "/fed/users/".to_owned();
    q_string.push_str(&name);

    let mut query = make_federated_request(
        awc::Client::get,
        user.host.to_string(),
        q_string.clone(),
        "{}".to_string(),
        None,
        Option::<()>::None,
    )?
    .await
    .map_err(|_| RouteError::ActixInternal)?;

    if !query.status().is_success() {
        Err(RouteError::NotFound)
    } else {
        let body = query.body().await?;

        let s_user: String =
            String::from_utf8(body.to_vec()).map_err(|_| RouteError::ActixInternal)?;

        let user_profile: UserProfile = serde_json::from_str(&s_user)?;

        Ok(user_profile)
    }
}
