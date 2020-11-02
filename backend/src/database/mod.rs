pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use self::models::{Post, NewPost};

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    // create database (if it doesn't already exist!)
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post<'a>(conn: &MysqlConnection, title: &'a str) {
    use schema::Posts;

    let new_post = NewPost {
        title: title,
    };
}
