#[derive(Queryable)]
pub struct User {
    pub id: i64,
    pub username: String,
}

#[derive(Queryable)]
pub struct Post {
    pub id: i64,
    pub uuid: String,
    pub title: String,
    pub author: i64,
    pub content: String,
    pub body: String,
    pub created: chrono::NaiveDate,
    pub modified: Option<chrono::NaiveDate>,
}

#[derive(Queryable)]
pub struct Community {
    pub id: i64,
    pub uuid: String,
    pub title: String,
    pub desc: String,
}

use super::schema::Posts;

#[derive(Insertable)]
#[table_name = "Posts"]
pub struct DBNewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
