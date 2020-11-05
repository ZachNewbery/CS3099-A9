use chrono::NaiveDateTime;
use either::Either;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub(crate) struct UserID {
    pub id: String,
    pub host: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) enum PostContentType {
    Text,
}

impl TryFrom<u64> for PostContentType {
    // TODO: Define error type here
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PostContentType::Text),
            _ => Err(()),
        }
    }
}

impl From<PostContentType> for u64 {
    fn from(value: PostContentType) -> Self {
        match value {
            PostContentType::Text => 0,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Community {
    id: String,
    title: String,
    description: String,
    admins: Vec<UserID>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct NewPost {
    pub parent: Uuid,
    pub title: String,
    #[serde(alias = "contentType")]
    pub content_type: PostContentType,
    pub body: String,
    pub author: UserID,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct UpdatePost {
    title: Option<String>,
    #[serde(alias = "contentType")]
    content_type: Option<PostContentType>,
    body: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Post {
    id: Uuid,
    parent: Option<Either<Uuid, String>>,
    children: Vec<Uuid>,
    #[serde(alias = "contentType")]
    content_type: PostContentType,
    body: String,
    author: UserID,
    created: NaiveDateTime,
    modified: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct PostTimeStamp {
    id: Uuid,
    modified: Option<NaiveDateTime>,
}
