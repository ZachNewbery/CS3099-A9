use crate::internal::authentication::authenticate;
use crate::DBPool;
use actix_web::{get, http, web, HttpRequest, HttpResponse, Result};
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

pub(crate) fn get_known_hosts() -> Vec<String> {
    let hosts = fs::File::open("known_hosts.txt").expect("file should open read only");
    let json: serde_json::Value =
        serde_json::from_reader(hosts).expect("could not parse known_hosts file");
    let vect = json.as_array().unwrap();

    vect.iter()
        .map(|s| s.as_str().unwrap().to_string())
        .collect()
}
