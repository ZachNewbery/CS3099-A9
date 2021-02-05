use chrono::{NaiveDateTime, Utc};

use crate::database::naive_date_time_now;
use crate::database::schema::{
    Communities, CommunitiesUsers, FederatedUsers, LocalUsers, Posts, Users,
};
use crate::federation::schemas::NewPost;
use crate::internal::authentication::generate_session;
use crate::internal::NewUser;

#[derive(Queryable, Identifiable, Debug, Clone)]
#[table_name = "Users"]
pub struct DatabaseUser {
    pub id: u64,
    pub username: String,
}

#[derive(Queryable, Identifiable, Associations, Debug, Clone)]
#[belongs_to(DatabaseUser, foreign_key = "userId")]
#[table_name = "LocalUsers"]
pub struct DatabaseLocalUser {
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
#[belongs_to(DatabaseUser, foreign_key = "userId")]
#[table_name = "FederatedUsers"]
pub struct DatabaseFederatedUser {
    pub id: u64,
    #[column_name = "userId"]
    pub user_id: u64,
    pub host: String,
}

#[derive(Queryable, Identifiable, Debug, Clone)]
#[table_name = "Communities"]
pub struct DatabaseCommunity {
    pub id: u64,
    pub name: String,
    pub desc: String,
    pub title: String,
}

#[derive(Queryable, Identifiable, Associations, Debug, Clone)]
#[table_name = "CommunitiesUsers"]
#[belongs_to(DatabaseCommunity, foreign_key = "communityId")]
pub struct DatabaseCommunitiesUser {
    pub id: u64,
    #[column_name = "communityId"]
    pub community_id: u64,
    #[column_name = "userId"]
    pub user_id: u64,
}

#[derive(Queryable, Identifiable, Associations, Debug, Clone)]
#[table_name = "Posts"]
#[belongs_to(DatabaseUser, foreign_key = "authorId")]
#[belongs_to(DatabaseCommunity, foreign_key = "communityId")]
#[belongs_to(DatabasePost, foreign_key = "parentId")]
pub struct DatabasePost {
    pub id: u64,
    pub uuid: String,
    pub title: String,
    #[column_name = "authorId"]
    pub author_id: u64,
    #[column_name = "contentType"]
    pub content_type: u64,
    // TODO: Check how we can convert this into a PostContentType
    pub body: String,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    #[column_name = "parentId"]
    pub parent_id: Option<u64>,
    #[column_name = "communityId"]
    pub community_id: u64,
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "Posts"]
pub struct DatabaseNewPost {
    pub uuid: String,
    pub title: String,
    #[column_name = "authorId"]
    pub author_id: u64,
    #[column_name = "contentType"]
    pub content_type: u64,
    // TODO: Check how we can convert this into a PostContentType
    pub body: String,
    pub created: NaiveDateTime,
    pub modified: NaiveDateTime,
    #[column_name = "parentId"]
    pub parent_id: Option<u64>,
    #[column_name = "communityId"]
    pub community_id: u64,
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "Users"]
pub struct DatabaseNewUser {
    pub username: String,
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "FederatedUsers"]
pub struct DatabaseNewFederatedUser {
    pub id: u64,
    pub host: String,
}

impl From<NewUser> for DatabaseNewUser {
    fn from(value: NewUser) -> Self {
        Self {
            username: value.username,
        }
    }
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "LocalUsers"]
pub struct DatabaseNewLocalUser {
    pub userId: u64,
    pub email: String,
    pub password: String,
    #[column_name = "createdAt"]
    pub created_at: NaiveDateTime,
    pub session: String,
}

impl From<(DatabaseUser, NewUser)> for DatabaseNewLocalUser {
    fn from(value: (DatabaseUser, NewUser)) -> Self {
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
