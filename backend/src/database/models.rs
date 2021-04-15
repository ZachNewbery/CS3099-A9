//! Database type definitions for serialization.
use chrono::NaiveDateTime;

use crate::database::naive_date_time_now;
use crate::database::schema::{
    Communities, CommunitiesUsers, FederatedUsers, LocalUsers, Markdown, Posts, Text, Users,
};

use crate::federation::schemas::User;
use crate::internal::authentication::generate_session;
use crate::internal::user::NewLocalUser;

/// Struct representing a row in the Users table in the database
#[derive(Queryable, Identifiable, Debug, Clone)]
#[table_name = "Users"]
pub struct DatabaseUser {
    /// Internal id of the User
    pub id: u64,
    /// Username of the User
    pub username: String,
}

/// Struct representing a row in the LocalUsers table in the database
#[derive(Queryable, Identifiable, Associations, Debug, Clone)]
#[belongs_to(DatabaseUser, foreign_key = "userId")]
#[table_name = "LocalUsers"]
pub struct DatabaseLocalUser {
    /// Internal id of the LocalUser
    pub id: u64,
    /// Internal id of the row in Users this LocalUser belongs to
    #[column_name = "userId"]
    pub user_id: u64,
    /// Email of the LocalUser
    pub email: String,
    /// Password of the LocalUser
    pub password: String,
    /// Time of account creation
    #[column_name = "createdAt"]
    pub created_at: NaiveDateTime,
    /// Last session of the LocalUser
    pub session: String,
    /// Optional bio of the LocalUser
    pub bio: Option<String>,
    /// Optional avatar URL of the LocalUser
    pub avatar: Option<String>,
}

/// Struct representing a row in the FederatedUsers table in the database
#[derive(Queryable, Identifiable, Associations, Debug, Clone)]
#[belongs_to(DatabaseUser, foreign_key = "userId")]
#[table_name = "FederatedUsers"]
pub struct DatabaseFederatedUser {
    /// Internal id of the FederatedUser
    pub id: u64,
    /// Internal id of the row in Users this FederatedUser belongs to
    #[column_name = "userId"]
    pub user_id: u64,
    /// Hostname of the FederatedUser
    pub host: String,
}

/// Struct representing a new User to be inserted into the Users table in the database
#[derive(Insertable, Debug, Clone)]
#[table_name = "Users"]
pub struct DatabaseNewUser {
    /// Username of the new User
    pub username: String,
}

/// Struct representing a new FederatedUser to be inserted into the FederatedUsers table in the database
#[derive(Insertable, Debug, Clone)]
#[table_name = "FederatedUsers"]
pub struct DatabaseNewFederatedUser {
    /// Internal id of the row in Users this FederatedUser belongs to
    #[column_name = "userId"]
    pub user_id: u64,
    /// Hostname of the FederatedUser
    pub host: String,
}

impl From<NewLocalUser> for DatabaseNewUser {
    fn from(value: NewLocalUser) -> Self {
        Self {
            username: value.username,
        }
    }
}

impl From<User> for DatabaseNewUser {
    fn from(value: User) -> Self {
        Self { username: value.id }
    }
}

/// Struct representing a new LocalUser to be inserted into the LocalUsers table in the database
#[derive(Insertable, Debug, Clone)]
#[table_name = "LocalUsers"]
pub struct DatabaseNewLocalUser {
    /// Internal id of the row in Users this LocalUser belongs to
    pub userId: u64,
    /// Email of the new LocalUser
    pub email: String,
    /// Password of the new LocalUser
    pub password: String,
    /// Time of creation of the LocalUser
    #[column_name = "createdAt"]
    pub created_at: NaiveDateTime,
    /// Current session of the LocalUser
    pub session: String,
}

impl From<(DatabaseUser, NewLocalUser)> for DatabaseNewLocalUser {
    fn from(value: (DatabaseUser, NewLocalUser)) -> Self {
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

impl From<(DatabaseUser, User)> for DatabaseNewFederatedUser {
    fn from(value: (DatabaseUser, User)) -> Self {
        let (user, new_user) = value;
        Self {
            user_id: user.id,
            host: new_user.host,
        }
    }
}

/// Struct representing a row within the Communities table in the database
#[derive(Queryable, Identifiable, Debug, Clone)]
#[table_name = "Communities"]
pub struct DatabaseCommunity {
    /// Internal id of the community
    pub id: u64,
    /// Unique name of the community
    pub name: String,
    /// Description of the community
    pub description: String,
    /// Title of the community
    pub title: String,
}

/// Struct representing a new Community to be added to the Communities table in the database
#[derive(Insertable, Debug, Clone)]
#[table_name = "Communities"]
pub struct DatabaseNewCommunity {
    /// Unique name of the community
    pub name: String,
    /// Description of the community
    pub description: String,
    /// Title of the community
    pub title: String,
}

/// Struct representing a row within the CommunitiesUsers table in the database
#[derive(Queryable, Identifiable, Associations, Debug, Clone)]
#[table_name = "CommunitiesUsers"]
#[belongs_to(DatabaseCommunity, foreign_key = "communityId")]
pub struct DatabaseCommunitiesUser {
    /// Internal id of the admin (community user)
    pub id: u64,
    /// Internal id of the community in the Communities table
    #[column_name = "communityId"]
    pub community_id: u64,
    /// Internal id of the user in the Users table
    #[column_name = "userId"]
    pub user_id: u64,
}

/// Struct representing a new admin to be inserted into the CommunitiesUsers table in the database
#[derive(Insertable, Debug, Clone)]
#[table_name = "CommunitiesUsers"]
pub struct DatabaseNewCommunitiesUser {
    /// Internal id of the Community in the Communities table
    #[column_name = "communityId"]
    pub community_id: u64,
    /// Internal id of the User in the Users table
    #[column_name = "userId"]
    pub user_id: u64,
}

/// Struct representing a row in the Posts table within the database
#[derive(Queryable, Identifiable, Associations, Debug, Clone)]
#[table_name = "Posts"]
#[belongs_to(DatabaseUser, foreign_key = "authorId")]
#[belongs_to(DatabaseCommunity, foreign_key = "communityId")]
#[belongs_to(DatabasePost, foreign_key = "parentId")]
pub struct DatabasePost {
    /// Internal id of the Post
    pub id: u64,
    /// Uuid of the Post
    pub uuid: String,
    /// Title of the Post (null for comments)
    pub title: Option<String>,
    /// Internal id of the author of the Post
    #[column_name = "authorId"]
    pub author_id: u64,
    /// Time of Post creation
    pub created: NaiveDateTime,
    /// Time of latest Post modification
    pub modified: NaiveDateTime,
    /// Internal id of the parent Post (null for top-level posts)
    #[column_name = "parentId"]
    pub parent_id: Option<u64>,
    /// Internal id of the community the Post belongs to
    #[column_name = "communityId"]
    pub community_id: u64,
    /// Boolean representing the Post being deleted or not
    pub deleted: bool,
}

/// Struct representing a Text content within a Post
#[derive(Queryable, Identifiable, Associations, Debug, Clone)]
#[table_name = "Text"]
#[belongs_to(DatabasePost, foreign_key = "postId")]
pub struct DatabaseText {
    /// Internal id of the text content
    pub id: u64,
    /// Actual text of the content object
    pub content: String,
    /// Internal id of the Post containing the text
    #[column_name = "postId"]
    pub post_id: u64,
}

/// Struct representing a Markdown content within a Post
#[derive(Queryable, Identifiable, Associations, Debug, Clone)]
#[table_name = "Markdown"]
#[belongs_to(DatabasePost, foreign_key = "postId")]
pub struct DatabaseMarkdown {
    /// Internal id of the markdown content
    pub id: u64,
    /// Actual markdown content of the content object
    pub content: String,
    /// Internal id of the Post containing the markdown
    #[column_name = "postId"]
    pub post_id: u64,
}

/// Struct representing a new Post to be inserted into Posts table in the database
#[derive(Insertable, Debug, Clone)]
#[table_name = "Posts"]
pub struct DatabaseNewPost {
    /// UUID belonging to the Post, generated upon creation
    pub uuid: String,
    /// Title of the post (null for comments)
    pub title: Option<String>,
    /// Internal id of the author of the Post
    #[column_name = "authorId"]
    pub author_id: u64,
    /// Time of Post creation
    pub created: NaiveDateTime,
    /// Time of latest Post modification (defaulted to creation time)
    pub modified: NaiveDateTime,
    /// Internal id of the parent of the Post (null for top-level posts)
    #[column_name = "parentId"]
    pub parent_id: Option<u64>,
    /// Internal id of the community the Post belongs to
    #[column_name = "communityId"]
    pub community_id: u64,
}
