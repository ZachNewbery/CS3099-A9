#![allow(non_snake_case)]

pub mod models;
pub mod schema;

use self::models::*;
use crate::federation::schemas::NewPost;
use crate::internal::authentication::Token;
use crate::internal::NewUser;
use crate::DBPool;
use actix_web::{web, HttpResponse};
use chrono::{NaiveDateTime, Utc};
use diesel::expression::count::count_star;
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
    let db_new_post = DBNewPost::from(new_post);

    conn.transaction::<(), diesel::result::Error, _>(|| {
        diesel::insert_into(Posts::table)
            .values(&db_new_post)
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

pub(crate) fn is_valid_session(
    conn: &MysqlConnection,
    token_ck: &Token,
) -> Result<bool, diesel::result::Error> {
    use crate::database::schema::LocalUsers::dsl::*;
    use crate::database::schema::Users::dsl::*;

    Ok(Users
        .inner_join(LocalUsers)
        .filter(userId.eq(&token_ck.user_id))
        .filter(session.eq(&token_ck.session))
        .select(count_star())
        .first::<i64>(conn)
        .optional()?
        .is_some())
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
pub(crate) fn get_local_user_by_email_password(
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
