#![allow(non_snake_case)]

pub mod models;
pub mod schema;
pub mod communities;

use self::models::*;
use crate::federation::schemas::NewPost;
use crate::internal::{LocalNewPost, NewUser};
use crate::DBPool;
use actix_web::{web, HttpResponse};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use uuid::Uuid;

fn naive_date_time_now() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)
}

// FIXME: This is here for MVP purposes
pub(crate) fn create_local_post(
    conn: &MysqlConnection,
    new_post: LocalNewPost,
    local_user: LocalUser,
) -> Result<(), diesel::result::Error> {
    use schema::Posts::dsl::*;

    let db_new_post = DBNewPost {
        uuid: Uuid::new_v4().to_string(),
        title: new_post.title,
        body: new_post.body,
        author: local_user.user_id,
        content_type: 0,
        created: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
        modified: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
    };

    diesel::insert_into(Posts)
        .values(db_new_post)
        .execute(conn)?;

    Ok(())
}


pub(crate) fn insert_federated_user(
    conn: &MysqlConnection,
    id_ck: &str,
    host_ck: &str,
) -> Result<(), diesel::result::Error> {
    conn.transaction::<(), diesel::result::Error, _>(|| {
        use crate::database::schema::FederatedUsers::dsl::*;
        use crate::database::schema::Users::dsl::*;

        let db_new_user = DBNewUser {
            username: id_ck.to_string(),
        };

        diesel::insert_into(Users)
            .values(db_new_user.clone())
            .execute(conn)?;

        let inserted_user: User = Users
            .filter(username.eq(&db_new_user.username))
            .first::<User>(conn)?;

        let db_new_fed_user = DBNewFedUser {
            id: inserted_user.id,
            host: host_ck.to_string(),
        };

        diesel::insert_into(FederatedUsers)
            .values(db_new_fed_user)
            .execute(conn)?;

        Ok(())
    })
}

pub(crate) fn insert_new_local_user(
    conn: &MysqlConnection,
    new_user: NewUser,
) -> Result<(), diesel::result::Error> {
    conn.transaction::<(), diesel::result::Error, _>(|| {
        use crate::database::schema::LocalUsers::dsl::*;
        use crate::database::schema::Users::dsl::*;

        let db_new_user: DBNewUser = new_user.clone().into();

        diesel::insert_into(Users)
            .values(db_new_user.clone())
            .execute(conn)?;

        // Unfortunately MySQL does not support RETURN statements.
        // We will have to make a second query to fetch the new user id.
        // TODO: Look into extracting function
        let inserted_user: User = Users
            .filter(username.eq(&db_new_user.username))
            .first::<User>(conn)?;

        let db_new_local_user: DBNewLocalUser = (inserted_user, new_user).into();

        diesel::insert_into(LocalUsers)
            .values(db_new_local_user)
            .execute(conn)?;

        Ok(())
    })
}

pub(crate) fn show_all_posts(conn: &MysqlConnection) -> Result<Vec<Post>, diesel::result::Error> {
    use schema::Posts::dsl::*;

    Posts.load::<Post>(conn)
}


// TODO: Refactor all other endpoints to use this!
pub fn get_conn_from_pool(
    pool: web::Data<DBPool>,
) -> actix_web::Result<PooledConnection<ConnectionManager<MysqlConnection>>> {
    pool.get()
        .map_err(|_| HttpResponse::ServiceUnavailable().finish().into())
}
