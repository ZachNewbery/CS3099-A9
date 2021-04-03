use actix_web::{get, http, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use std::fs;

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

#[get("/servers")]
pub(crate) async fn discover() -> Result<HttpResponse> {
    let file = fs::File::open("known_hosts.txt").expect("file should open read only");
    let json: serde_json::Value =
        serde_json::from_reader(file).expect("file should be proper JSON");

    Ok(HttpResponse::Ok()
        .header(http::header::CONTENT_TYPE, "application/json")
        .body(json))
}
