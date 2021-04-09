use crate::database::actions::post::PostInformation;
use crate::util::route_error::RouteError;
use chrono::serde::{ts_seconds, ts_seconds_option};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ContentType {
    Text,
    Markdown,
    #[serde(other)]
    Unsupported,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct InnerContent {
    #[serde(default = "default_string")]
    pub text: String,
}

fn default_string() -> String {
    "unsupported content type".to_string()
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
    pub title: String,
    pub content: Vec<HashMap<ContentType, serde_json::Value>>,
    pub user_id: String,
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
            title: Some(post.post.title),
            content: post.content,
            author: (post.user, post.user_details).into(),
            modified: DateTime::<Utc>::from_utc(post.post.modified, Utc),
            created: DateTime::<Utc>::from_utc(post.post.created, Utc),
        })
    }
}
