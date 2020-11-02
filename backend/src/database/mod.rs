#![allow(non_snake_case)]

pub mod models;
pub mod schema;

use self::models::NewPost;
use diesel::prelude::*;

pub fn create_post<'a>(conn: &MysqlConnection, title: &'a str, body: &'a str) {
    use schema::Posts;

    let new_post = NewPost {
        title,
        body,
    };

    diesel::insert_into(Posts::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post");
}
