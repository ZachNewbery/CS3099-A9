use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub(crate) struct UserID {
    user_id: String,
    server_id: String,
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
    parent: String, // Should be UUID v4?
    title: String,
    body: String,
    author: UserID,
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
