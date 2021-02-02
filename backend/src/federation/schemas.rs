use std::convert::TryFrom;

use actix_web::dev::HttpResponseBuilder;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use chrono::NaiveDateTime;
use either::Either;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug, Copy, Clone)]
#[error("bad request")]
pub enum FederationSchemaError {
    #[error("unknown post content type")]
    PostContentType,
}

impl ResponseError for FederationSchemaError {
    fn status_code(&self) -> StatusCode {
        StatusCode::BAD_REQUEST
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code()).finish()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct User {
    pub id: String,
    pub host: String,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) enum PostContentType {
    Text,
}

impl TryFrom<u64> for PostContentType {
    type Error = FederationSchemaError;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PostContentType::Text),
            _ => Err(FederationSchemaError::PostContentType),
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Community {
    id: String,
    title: String,
    description: String,
    admins: Vec<User>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct NewPost {
    pub community: String,
    pub parent_post: Option<Uuid>,
    pub title: String,
    pub content: Vec<String>, // TODO: PostContentText or PostContentMarkdown
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UpdatePost {
    title: Option<String>,
    content_type: Option<PostContentType>,
    body: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostTimeStamp {
    id: Uuid,
    modified: Option<NaiveDateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Post {
    id: Uuid,
    community: String,
    parent_post: Uuid,
    children: Vec<Uuid>,
    title: String,
    content: Vec<String>,
    // TODO: PostContentText or PostContentMarkdown
    author: User,
    modified: NaiveDateTime,
    created: NaiveDateTime,
}
