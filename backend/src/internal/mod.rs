//! Internal API Implementation for frontend communication.
use crate::internal::authentication::authenticate;
use crate::DBPool;
use actix_web::{get, http, web, HttpRequest, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use std::fs;

pub mod authentication;
pub mod communities;
pub mod posts;
pub mod user;

/// Struct abstracting over local and federated communities
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum LocatedCommunity {
    /// Local Community details
    Local { 
        /// Name of the community (id as per supergroup spec)
        id: String 
    },
    /// Federated Community details
    Federated { 
        /// Name of the community (id as per supergroup spec)
        id: String, 
        /// Hostname that the community is stored on
        host: String 
    },
}

/// Internal endpoint to return all the currently known federated hosts
#[get("/servers")]
pub(crate) async fn discover(
    pool: web::Data<DBPool>,
    request: HttpRequest,
) -> Result<HttpResponse> {
    let (_, _) = authenticate(pool, request)?;
    let file = fs::File::open("known_hosts.txt").expect("file should open read only");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");

    Ok(HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(json))
}

/// Returns all our known hosts as a vector of Strings
pub(crate) fn get_known_hosts() -> Vec<String> {
    let hosts = fs::File::open("known_hosts.txt").expect("file should open read only");
    let json: serde_json::Value =
        serde_json::from_reader(hosts).expect("could not parse known_hosts file");
    let vect = json.as_array().unwrap();

    vect.iter()
        .map(|s| s.as_str().unwrap().to_string())
        .collect()
}
