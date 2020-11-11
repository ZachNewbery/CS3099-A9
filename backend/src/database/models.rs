use crate::database::naive_date_time_now;
use crate::database::schema::{Communities, FederatedUsers, LocalUsers, Posts, Users};
use crate::federation::schemas::NewPost;
use crate::internal::authentication::generate_session;
use crate::internal::NewUser;
use chrono::{NaiveDateTime, Utc};

#[derive(Queryable, Identifiable, Debug, Clone)]
#[table_name = "Users"]
pub struct User {
    pub id: u64,
    pub username: String,
}

#[derive(Queryable, Identifiable, Associations, Debug, Clone)]
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

#[derive(Queryable, Identifiable, Associations, Debug, Clone)]
#[belongs_to(User, foreign_key = "userId")]
#[table_name = "FederatedUsers"]
pub struct FederatedUser {
    pub id: u64,
    #[column_name = "userId"]
    pub user_id: u64,
    pub host: String,
}

#[derive(Queryable, Identifiable, Associations, Debug, Clone)]
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

#[derive(Queryable, Identifiable, Debug, Clone)]
#[table_name = "Communities"]
pub struct Community {
    pub id: u64,
    pub uuid: String,
    pub title: String,
    pub desc: String,
}

#[derive(Insertable, Debug, Clone)]
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

#[derive(Insertable, Debug, Clone)]
#[table_name = "Users"]
pub struct DBNewUser {
    pub username: String,
}

impl From<NewUser> for DBNewUser {
    fn from(value: NewUser) -> Self {
        Self {
            username: value.username,
        }
    }
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "LocalUsers"]
pub struct DBNewLocalUser {
    pub userId: u64,
    pub email: String,
    pub password: String,
    #[column_name = "createdAt"]
    pub created_at: NaiveDateTime,
    pub session: String,
}

impl From<(User, NewUser)> for DBNewLocalUser {
    fn from(value: (User, NewUser)) -> Self {
        let (user, new_user) = value;
        Self {
            userId: user.id,
            email: new_user.email,
            password: new_user.password,
            created_at: naive_date_time_now(),
            session: generate_session(),
        }
    }
}
