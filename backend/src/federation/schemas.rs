use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

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
    pub parent: String, // Should be UUID v4?
    pub title: String,
    pub content_type: PostContentType,
    pub body: String,
    pub author: UserID,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct UpdatePost {
    title: String,
    body: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct Post {
    id: String,            // Should be UUID v4?
    children: Vec<String>, // Should be a vec of UUID v4?
    content_type: PostContentType,
    body: String,
    author: UserID,
    modified: u64, // Should be timestamp?
    created: u64,  // Should be timestamp?
}

#[derive(Serialize, Deserialize)]
pub(crate) struct PostTimeStamp {
    id: String,    // Should be UUID v4?
    modified: u64, // Should be timestamp?
}
