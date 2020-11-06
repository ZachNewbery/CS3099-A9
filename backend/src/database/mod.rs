#![allow(non_snake_case)]

pub mod models;
pub mod schema;

use self::models::*;
use crate::federation::schemas::NewPost;
use crate::internal::authentication::Token;
use diesel::prelude::*;

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
    _conn: &MysqlConnection,
    _user: &User,
    _session: String,
) -> Result<(), diesel::result::Error> {
    // TODO: Write database action that updates session
    unimplemented!()
}

pub(crate) fn is_valid_session(
    _conn: &MysqlConnection,
    _token: &Token,
) -> Result<bool, diesel::result::Error> {
    // TODO: Write database action that validates a session
    unimplemented!()
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
