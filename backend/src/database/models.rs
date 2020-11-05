#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub author: i32,
    pub content: String,
    pub uuid: String,
    pub title: String,
    pub body: String,
}

#[derive(Queryable)]
pub struct Community {
    pub id: i32,
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
