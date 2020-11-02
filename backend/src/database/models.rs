#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub uuid: String,
    pub title: String,
    pub body: String,
}

#[derive(Queryable)]
pub struct Community {
    pub id: i32,
    pub uuid: String,
    pub title: String,
}

use super::schema::Posts;

#[derive(Insertable)]
#[table_name = "Posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}
