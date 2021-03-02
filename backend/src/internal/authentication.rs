use actix_web::error::BlockingError;
use actix_web::http::header::Header as ActixHeader;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};

use chrono::Utc;
use crypto::{digest::Digest, sha2::Sha512};

use http_signature_normalization_actix::prelude::*;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};

use serde::{Deserialize, Serialize};

use std::time::SystemTime;
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

pub async fn request_wrapper() -> String {
    let _config = Config::default();
    let mut digest = Sha512::new();

    // hash body of HTTP request (need to work out how to do for post requests!)
    digest.input_str("");
    let hex = hex::decode(digest.result_str()).expect("Hex string decoded");
    let _digest_header = base64::encode(hex);
    let date = SystemTime::now().into();

    // create request to be signed
    let req = awc::Client::new()
        .get("https://cs3099user-a7.host.cs.st-andrews.ac.uk/fed/posts")
        .header("User-Agent", "Actix Web")
        // .header("Digest", ["sha-512=", &digest_header].join(""))
        .set(actix_web::http::header::Date(date))
        .send()
        .await;
    // builder().connector(
    //     awc::Connector::new()
    //             .timeout(Duration::from_secs(20))
    //             .finish(),
    // )
    // .finish()
    // construct string as per supergroup protocol
    // let header_map = req.headers();
    // let mut string = String::new();
    // string.push_str(&format!("(request-target): get {}\n", "/fed/posts"));
    // string.push_str(&format!("host: {}\n", "https://cs3099user-a9.host.cs.st-andrews.ac.uk/"));
    // string.push_str(&format!("client-host: {}\n", "https://cs3099user-a7.host.cs.st-andrews.ac.uk"));
    // string.push_str(&format!("date: {}\n", date));
    // string.push_str(&format!("digest: SHA-512={}", digest_header));

    // // obtain private key from file and sign string
    // let pkey = PKey::private_key_from_pem(&fs::read("fed_auth.pem").expect("reading key")).expect("Getting private key.");
    // let mut signer = Signer::new(MessageDigest::sha512(), &pkey).unwrap();
    // signer.update(string.as_bytes()).unwrap();

    // // base64 encode string
    // let signature = signer.sign_to_vec().unwrap();
    // let encoded_sign = base64::encode(signature);

    // // append header to request
    // let mut header_val = String::new();
    // header_val.push_str("keyId=\"rsa-global\",algorithm=\"hs2019\",headers=\"(request-target) host client-host date digest\",signature=");
    // header_val.push_str(&encoded_sign);
    // let new_req = req.header("Signature", header_val);

    // send request?
    // let response = new_req.timeout(Duration::from_secs(20)).send().await;

    println!("Response: {:?}", req);

    return "done".to_string();
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
