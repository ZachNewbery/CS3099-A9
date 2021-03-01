use serde::{Deserialize, Serialize};

pub mod authentication;
pub mod communities;
pub mod posts;
pub mod user;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum LocatedCommunity {
    Local { id: String },
    Federated { id: String, host: String },
}
