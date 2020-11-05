#![allow(non_snake_case)]

pub mod models;
pub mod schema;

use self::models::*;
use crate::federation::schemas::NewPost;
use diesel::prelude::*;

#[allow(dead_code)]
pub(crate) fn create_post(
    _conn: &MysqlConnection,
    _new_post: &NewPost,
) -> Result<(), diesel::result::Error> {
    use schema::Posts;

    let new_post = DBNewPost {
        title: &_new_post.title,
        body: &_new_post.body,
    };

    diesel::insert_into(Posts::table)
        .values(&new_post)
        .execute(_conn)?;

    Ok(())
}

#[allow(dead_code)]
pub(crate) fn show_posts(_conn: &MysqlConnection) -> Result<(), diesel::result::Error> {
    use schema::Posts::dsl::*;

    // let result = Posts.limit(5).load::<Post>(_conn).expect("Error Getting Posts.");

    Ok(())
}
