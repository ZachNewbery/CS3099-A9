#![allow(non_snake_case)]

pub mod models;
pub mod schema;

use crate::federation::schemas::NewPost;
use diesel::prelude::*;

#[allow(dead_code)]
pub(crate) fn create_post(
    _conn: &MysqlConnection,
    _new_post: &NewPost,
) -> Result<(), diesel::result::Error> {
    // use schema::Posts;

    // let new_post = NewPost {
    //     title,
    //     body,
    // };

    // diesel::insert_into(Posts::table)
    //     .values(&new_post)
    //     .execute(conn)?;

    Ok(())
}
