use serde::{Deserialize, Serialize};

pub mod authentication;
pub mod posts;
pub mod user;
pub mod communities;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LocatedCommunity {
    Local { id: String },
    Federated { id: String, host: String },
}
