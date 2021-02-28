use actix_web::http::header::Header as ActixHeader;
use actix_web::{client::Client, client::ClientRequest, error::BlockingError};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};
use base64::encode;
use chrono::Utc;
use http_signature_normalization_actix::prelude::*;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};
use std::time::{Duration, SystemTime};
use uuid::Uuid;

use crate::database::actions::local::validate_session;
use crate::database::get_conn_from_pool;
use crate::database::models::DatabaseLocalUser;
use crate::DBPool;

pub static JWT_SECRET_KEY: [u8; 16] = *include_bytes!("../../jwt_secret.key");

// Timeout of one week in seconds
const TIMEOUT: i64 = 60 * 60 * 24 * 7;

pub fn generate_session() -> String {
    Uuid::new_v4().to_simple().to_string()
}

// pub fn validate_token() -> jsonwebtoken::errors::Result<TokenData<Token>> {
//     unimplemented!()
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    #[serde(rename = "iat")]
    pub issued_at: i64,
    #[serde(rename = "exp")]
    pub expiration: i64,
    // Claims
    // This is the id (i.e. pk of LocalUsers)
    pub id: u64,
    // Session
    pub session: String,
}

impl Token {
    pub fn new(id: u64, session: &str) -> Self {
        Self {
            issued_at: Utc::now().timestamp(),
            expiration: Utc::now().timestamp() + TIMEOUT,
            id,
            session: session.to_string(),
        }
    }

    pub fn generate_token(&self) -> jsonwebtoken::errors::Result<String> {
        jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(&JWT_SECRET_KEY),
        )
    }

    pub fn decode_token(token: &str) -> jsonwebtoken::errors::Result<TokenData<Token>> {
        jsonwebtoken::decode(
            token,
            &DecodingKey::from_secret(&JWT_SECRET_KEY),
            &Validation::default(),
        )
    }
}

pub fn authenticate(
    pool: web::Data<DBPool>,
    request: HttpRequest,
) -> actix_web::Result<(TokenData<Token>, DatabaseLocalUser)> {
    let conn = get_conn_from_pool(pool)?;

    let auth = Authorization::<Bearer>::parse(&request)?.into_scheme();

    let token =
        Token::decode_token(auth.token()).map_err(|_| HttpResponse::Unauthorized().finish())?;

    let local_user = validate_session(&conn, token.claims.id, &token.claims.session)
        .map_err(|_| HttpResponse::InternalServerError().finish())?
        .ok_or_else(|| HttpResponse::Unauthorized().finish())?;

    Ok((token, local_user))
}

pub async fn request_wrapper(
    request: ClientRequest,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let config = Config::default();
    let digest = Sha512::new();

    let mut response = request
        .header("User-Agent", "Actix Web")
        .set(actix_web::http::header::Date(SystemTime::now().into()))
        .signature_with_digest(config, "my-key-id", digest, "", |s| {
            Ok(base64::encode(s)) as Result<_, MyError>
        })
        .await?;
    // .send()
    // .await
    // .map_err(|e| {
    //     eprintln!("Error, {}", e);
    //     MyError::SendRequest
    // })?;
    unimplemented!();
}

#[derive(Debug, thiserror::Error)]
pub enum MyError {
    #[error("Failed to read header, {0}")]
    Convert(#[from] ToStrError),

    #[error("Failed to create header, {0}")]
    Header(#[from] InvalidHeaderValue),

    #[error("Failed to send request")]
    SendRequest,

    #[error("Failed to retrieve request body")]
    Body,

    #[error("Failed to prepare signing")]
    Sign(#[from] PrepareSignError),

    #[error("Blocking operation was canceled")]
    Canceled,
}

impl From<BlockingError<MyError>> for MyError {
    fn from(b: BlockingError<MyError>) -> Self {
        match b {
            BlockingError::Error(e) => e,
            _ => MyError::Canceled,
        }
    }
}
