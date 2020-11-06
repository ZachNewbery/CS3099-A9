use crate::database::schema::{Communities, FederatedUsers, LocalUsers, Posts, Users};
use crate::federation::schemas::NewPost;
use chrono::{NaiveDateTime, Utc};

#[derive(Queryable, Identifiable)]
#[table_name = "Users"]
pub struct User {
    pub id: u64,
    pub username: String,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(User, foreign_key = "userId")]
#[table_name = "LocalUsers"]
pub struct LocalUser {
    pub id: u64,
    #[column_name = "userId"]
    pub user_id: u64,
    pub email: String,
    pub password: String,
    #[column_name = "createdAt"]
    pub created_at: NaiveDateTime,
    pub session: String,
}

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(User, foreign_key = "userId")]
#[table_name = "FederatedUsers"]
pub struct FederatedUser {
    pub id: u64,
    #[column_name = "userId"]
    pub user_id: u64,
    pub host: String,
}

#[derive(Queryable, Identifiable, Associations)]
#[table_name = "Posts"]
#[belongs_to(User, foreign_key = "author")]
pub struct Post {
    pub id: u64,
    pub uuid: String,
    pub title: String,
    pub author: u64,
    #[column_name = "contentType"]
    pub content_type: u64, // TODO: Check how we can convert this into a PostContentType
    pub body: String,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
}

#[derive(Queryable, Identifiable)]
#[table_name = "Communities"]
pub struct Community {
    pub id: u64,
    pub uuid: String,
    pub title: String,
    pub desc: String,
}

#[derive(Insertable)]
#[table_name = "Posts"]
pub struct DBNewPost {
    pub uuid: String,
    pub title: String,
    pub body: String,
    pub author: u64,
    #[column_name = "contentType"]
    pub content_type: u64,
    pub created: NaiveDateTime,
}

// TODO: Replace the placeholder user id!
impl From<NewPost> for DBNewPost {
    fn from(value: NewPost) -> Self {
        Self {
            uuid: ::uuid::Uuid::new_v4().to_string(),
            title: value.title,
            body: value.body,
            author: 0,
            content_type: value.content_type.into(),
            created: NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0),
        }
    }
}
