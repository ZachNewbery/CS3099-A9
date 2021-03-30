use crate::database::models::{DatabaseFederatedUser, DatabaseLocalUser, DatabaseUser};
use crate::federation::schemas::User;

pub mod route_error;

pub const HOSTNAME: &str = "cs3099-a9.host.cs.st-andrews.ac.uk";

#[derive(Clone, Debug)]
pub enum UserDetail {
    Local(DatabaseLocalUser),
    Federated(DatabaseFederatedUser),
}

impl From<(DatabaseUser, UserDetail)> for User {
    fn from(value: (DatabaseUser, UserDetail)) -> Self {
        match value.1 {
            UserDetail::Local(_) => Self {
                id: value.0.username.to_string(),
                host: HOSTNAME.to_string(),
            },
            UserDetail::Federated(f) => Self {
                id: value.0.username.to_string(),
                host: f.host,
            },
        }
    }
}
