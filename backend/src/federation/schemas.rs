use crate::database::actions::post::PostInformation;
use crate::util::route_error::RouteError;
use chrono::serde::{ts_milliseconds, ts_milliseconds_option};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::rust::string_empty_as_none;
// use serde::Deserializer;
// use serde::de::{SeqAccess, Visitor};
use std::convert::TryFrom;
// use std::marker::PhantomData;
// use std::fmt::Formatter;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct User {
    pub id: String,
    pub host: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum ContentType {
    Text {
        text: String,
    },
    Markdown {
        text: String,
    },
    #[serde(other)]
    Unsupported
}

#[derive(Serialize, Deserialize, Debug)]
struct Unsupported;

// fn deserialize_vec_content_type<'de, D>(deserializer: D) -> Result<Vec<ContentType>, D::Error>
//     where
//         D: Deserializer<'de>,
// {
//     struct VecContentType(PhantomData<fn() -> Vec<ContentType>>);

//     impl<'de> Visitor<'de> for VecContentType {
//         type Value = Vec<ContentType>;

//         fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
//             formatter.write_str("Array of ContentType")
//         }

//         fn visit_seq<S>(self, mut seq: S) -> Result<Vec<ContentType>, S::Error>
//             where
//                 S: SeqAccess<'de>,
//         {
//             let mut field_kinds: Vec<ContentType> = Vec::new();

//             loop {
//                 match seq.next_element() {
//                     Ok(Some(element)) => field_kinds.push(element),
//                     Ok(None) => break, // end of sequence
//                     Err(_) => field_kinds.push(ContentType::Text{
//                         text: "content not supported.".to_string()
//                     }),
//                 }
//             }

//             Ok(field_kinds)
//         }
//     }

//     deserializer.deserialize_seq(VecContentType(PhantomData))
// }

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
    pub content: Vec<ContentType>,
    pub user_id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct UpdatePost {
    pub title: Option<String>,
    pub content_type: Option<ContentType>,
    pub body: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PostTimeStamp {
    id: Uuid,
    #[serde(with = "ts_milliseconds_option")]
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
    // #[serde(deserialize_with="deserialize_vec_content_type")]
    pub(crate) content: Vec<ContentType>,
    pub(crate) author: User,
    #[serde(with = "ts_milliseconds")]
    pub(crate) modified: DateTime<Utc>,
    #[serde(with = "ts_milliseconds")]
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
