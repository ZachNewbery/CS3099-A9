//! Federated schema implementations for serialization
use crate::database::actions::post::PostInformation;
use crate::util::route_error::RouteError;
use crate::util::route_error::RouteError::{BadPostContent, UnsupportedContentType};
use chrono::serde::{ts_seconds, ts_seconds_option};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_with::rust::string_empty_as_none;
use std::collections::HashMap;
use std::convert::TryFrom;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct User {
    pub id: String,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Community {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) description: String,
    pub(crate) admins: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NewPost {
    pub community: String,
    pub parent_post: Option<Uuid>,
    pub title: Option<String>,
    pub content: Vec<HashMap<ContentType, serde_json::Value>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UpdatePost {
    pub title: Option<String>,
    pub content_type: Option<HashMap<ContentType, serde_json::Value>>,
    pub body: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostTimeStamp {
    id: Uuid,
    #[serde(with = "ts_seconds_option")]
    modified: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Post {
    pub(crate) id: Uuid,
    pub(crate) community: String,
    #[serde(deserialize_with = "string_empty_as_none::deserialize")]
    pub(crate) parent_post: Option<Uuid>,
    pub(crate) children: Vec<Uuid>,
    pub(crate) title: Option<String>,
    pub(crate) content: Vec<HashMap<ContentType, serde_json::Value>>,
    pub(crate) author: User,
    #[serde(with = "ts_seconds")]
    pub(crate) modified: DateTime<Utc>,
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
