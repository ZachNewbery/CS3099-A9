#![allow(non_snake_case)]

pub mod models;
pub mod schema;

use self::models::*;
use crate::federation::schemas::NewPost;
use crate::internal::NewUser;
use crate::DBPool;
use actix_web::{web, HttpResponse};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};

fn naive_date_time_now() -> NaiveDateTime {
    NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0)
}

pub(crate) fn create_federated_post(
    conn: &MysqlConnection,
    new_post: NewPost,
) -> Result<(), diesel::result::Error> {
    use schema::Posts;

    // TODO: Write database action to insert-or-get user, remove default
    if get_federated_user(&conn, &new_post.author.id, &new_post.author.host)?.is_none() {
        // Update both Users and FederatedUsers table.
        insert_federated_user(&conn, &new_post.author.id, &new_post.author.host)?;
    }

    let db_new_post = DBNewPost::from(new_post);

    conn.transaction::<(), diesel::result::Error, _>(|| {
        diesel::insert_into(Posts::table)
            .values(&db_new_post)
            .execute(conn)?;

        Ok(())
    })
}

pub(crate) fn get_federated_user(
    conn: &MysqlConnection,
    username_ck: &str,
    host_ck: &str,
) -> Result<Option<FederatedUser>, diesel::result::Error> {
    use crate::database::schema::FederatedUsers::dsl::*;
    use crate::database::schema::Users::dsl::*;

    Ok(
        Users
            .inner_join(FederatedUsers)
            .filter(username.eq(username_ck))
            .filter(host.eq(host_ck))
            .select(FederatedUsers::all_columns())
            .first::<FederatedUser>(conn)
            .optional()?
    )
}

pub(crate) fn insert_federated_user(
    conn: &MysqlConnection,
    id_ck: &str,
    host_ck: &str,
) -> Result<(), diesel::result::Error> {
    conn.transaction::<(), diesel::result::Error, _>(|| {
        use crate::database::schema::FederatedUsers::dsl::*;
        use crate::database::schema::Users::dsl::*;

        // how do we get usernames from UserID struct?
        // TODO: replace username placeholder
        let db_new_user = DBNewUser {
            username: "placeholder".to_string(),
        };

        diesel::insert_into(Users)
            .values(db_new_user.clone())
            .execute(conn)?;

        let inserted_user: User = Users
            .filter(username.eq(&db_new_user.username))
            .first::<User>(conn)?;

        // TODO: Fix user id vs row id.
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

pub(crate) fn update_session(
    conn: &MysqlConnection,
    user: &LocalUser,
    new_session: &str,
) -> Result<(), diesel::result::Error> {
    use crate::database::schema::LocalUsers::dsl::*;

    diesel::update(LocalUsers.filter(id.eq(user.id)))
        .set(session.eq(new_session))
        .execute(conn)?;
    Ok(())
}

pub(crate) fn validate_session(
    conn: &MysqlConnection,
    id_ck: u64,
    session_ck: &str,
) -> Result<Option<LocalUser>, diesel::result::Error> {
    use crate::database::schema::LocalUsers::dsl::*;

    Ok(LocalUsers
        .filter(id.eq(id_ck))
        .filter(session.eq(session_ck))
        .first::<LocalUser>(conn)
        .optional()?)
}

pub(crate) fn get_local_user(
    conn: &MysqlConnection,
    username_ck: &str,
    email_ck: &str,
) -> Result<Option<LocalUser>, diesel::result::Error> {
    use crate::database::schema::LocalUsers::dsl::*;
    use crate::database::schema::Users::dsl::*;

    Ok(Users
        .inner_join(LocalUsers)
        .filter(username.eq(username_ck))
        .filter(email.eq(email_ck))
        .select(LocalUsers::all_columns())
        .first::<LocalUser>(conn)
        .optional()?)
}

// FIXME: I cannot emphasize just how insecure this is. MUST fix before pushing to production
pub(crate) fn login_local_user(
    conn: &MysqlConnection,
    email_ck: &str,
    password_ck: &str,
) -> Result<Option<LocalUser>, diesel::result::Error> {
    use crate::database::schema::LocalUsers::dsl::*;

    Ok(LocalUsers
        .filter(email.eq(email_ck))
        .filter(password.eq(password_ck))
        .first::<LocalUser>(conn)
        .optional()?)
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

// #[allow(dead_code)]
// pub(crate) fn show_posts(_conn: &MysqlConnection) -> Result<(), diesel::result::Error> {
//     use schema::Posts::dsl::*;
//
//     let result = Posts
//         .limit(5)
//         .load::<Post>(_conn)
//         .expect("Error Getting Posts.");
//
//     Ok(())
// }

// TODO: Refactor all other endpoints to use this!
pub fn get_conn_from_pool(
    pool: web::Data<DBPool>,
) -> actix_web::Result<PooledConnection<ConnectionManager<MysqlConnection>>> {
    pool.get()
        .map_err(|_| HttpResponse::ServiceUnavailable().finish().into())
}
