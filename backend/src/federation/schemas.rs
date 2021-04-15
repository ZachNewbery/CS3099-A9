//! Federated schema implementations for serialization
use crate::database::actions::post::PostInformation;
use crate::util::route_error::RouteError;
use crate::util::route_error::RouteError::{BadPostContent, UnsupportedContentType};
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::rust::string_empty_as_none;
use std::collections::HashMap;
use std::convert::TryFrom;
use uuid::Uuid;

/// Represents a User object passed via JSON request bodies
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct User {
    /// Username of the user
    pub id: String,
    /// Hostname the user belongs to
    pub host: String,
}

/// Enum representing the currently supported content types
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ContentType {
    /// Text content type
    Text,
    /// Markdown content type
    Markdown,
    /// Unsupported content type (all values other than the above are mapped to this)
    #[serde(other)]
    Unsupported,
}

/// Struct representing the content of a content type object
#[derive(Clone, Serialize, Deserialize)]
pub enum DatabaseContentType {
    /// Text content object
    Text { 
        /// Actual text content
        text: String 
    },
    /// Markdown content object
    Markdown { 
        /// Actual markdown content
        text: String
    },
}

impl TryFrom<&HashMap<ContentType, serde_json::Value>> for DatabaseContentType {
    type Error = RouteError;

    fn try_from(value: &HashMap<ContentType, Value>) -> Result<Self, Self::Error> {
        let ct = match value.iter().next() {
            Some((k, v)) => {
                match k {
                    ContentType::Text => {
                        // Text: field is text
                        DatabaseContentType::Text {
                            text: v
                                .get("text")
                                .ok_or(BadPostContent)?
                                .as_str()
                                .ok_or(BadPostContent)?
                                .to_string(),
                        }
                    }
                    ContentType::Markdown => DatabaseContentType::Markdown {
                        text: v
                            .get("text")
                            .ok_or(BadPostContent)?
                            .as_str()
                            .ok_or(BadPostContent)?
                            .to_string(),
                    },
                    ContentType::Unsupported => return Err(UnsupportedContentType),
                }
            }
            None => return Err(BadPostContent),
        };

        Ok(ct)
    }
}

/// Struct representing the response recieved by federated hosts when finding communities
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Community {
    /// Name of the community (id as per supergroup spec)
    pub(crate) id: String,
    /// Title of the community
    pub(crate) title: String,
    /// Description of the community
    pub(crate) description: String,
    /// Array of admins of the community
    pub(crate) admins: Vec<User>,
}

/// Struct representing the JSON body when a federated host creates a new post
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NewPost {
    /// Name of the community the post is being created in
    pub community: String,
    /// Optional UUID of the parent post of the Post
    pub parent_post: Option<Uuid>,
    /// Title of the new post (null for comments)
    pub title: Option<String>,
    /// Array of content of the new post
    pub content: Vec<HashMap<ContentType, serde_json::Value>>,
}

/// Struct reprsenting a request body to edit a post
#[derive(Clone, Serialize, Deserialize, Debug)]
pub(crate) struct EditPost {
    /// Optional new title to be set
    pub title: Option<String>,
    /// Optional new content to be set
    pub content: Option<Vec<HashMap<ContentType, serde_json::Value>>>,
}

/// Struct representing all the required attributes to form correct responses containing Posts as per the supergroup protocol
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Post {
    /// UUID of the Post
    pub(crate) id: Uuid,
    /// Name of the community the Post belongs to
    pub(crate) community: String,
    /// Optional UUID of the parent post of the Post
    #[serde(deserialize_with = "string_empty_as_none::deserialize")]
    pub(crate) parent_post: Option<Uuid>,
    /// Array of children of the Post
    pub(crate) children: Vec<Uuid>,
    /// Title of the Post (null for comments)
    pub(crate) title: Option<String>,
    /// Array of content of the Post
    pub(crate) content: Vec<HashMap<ContentType, serde_json::Value>>,
    /// User details for the author of the Post
    pub(crate) author: User,
    /// Time of last post modification
    #[serde(with = "ts_seconds")]
    pub(crate) modified: DateTime<Utc>,
    /// Time of post creation
    #[serde(with = "ts_seconds")]
    pub(crate) created: DateTime<Utc>,
}

impl TryFrom<(PostInformation, Option<Vec<PostInformation>>)> for Post {
    type Error = RouteError;

    fn try_from(
        value: (PostInformation, Option<Vec<PostInformation>>),
    ) -> Result<Self, Self::Error> {
        let (post, children) = value;
        Ok(Post {
            id: post.post.uuid.parse()?,
            community: post.community.name,
            parent_post: post.parent.map(|u| u.uuid.parse()).transpose()?,
            children: children
                .unwrap_or_default()
                .into_iter()
                .map(|p| Ok(p.post.uuid.parse()?))
                .collect::<Result<Vec<_>, RouteError>>()?,
            title: post.post.title,
            content: post.content,
            author: (post.user, post.user_details).into(),
            modified: DateTime::<Utc>::from_utc(post.post.modified, Utc),
            created: DateTime::<Utc>::from_utc(post.post.created, Utc),
        })
    }
}
