use actix_web::error::BlockingError;
use actix_web::http::header::Header as ActixHeader;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};

use chrono::Utc;
use crypto::{digest::Digest, sha2::Sha512};

use http_signature_normalization_actix::prelude::*;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use openssl::hash::*;
use openssl::pkey::*;
use openssl::rsa::Padding;
use openssl::sign::*;

use serde::{Deserialize, Serialize};

use std::fs;
use std::str;
use std::time::SystemTime;
use uuid::Uuid;

use crate::database::actions::local::validate_session;
use crate::database::get_conn_from_pool;
use crate::database::models::DatabaseLocalUser;
use crate::util::route_error::RouteError;
use crate::DBPool;
use awc::{ClientRequest, SendClientRequest};

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

pub async fn make_federated_request<T>(
    rq_ctor: fn(&awc::Client, url: String) -> ClientRequest,
    host: String,
    endpoint: String,
    body: T,
    uid: Option<String>,
) -> Result<SendClientRequest, RouteError>
where
    T: Serialize,
{
    let mut digest = Sha512::new();

    // hash body of HTTP request (need to work out how to do for post requests!)
    digest.input_str(&serde_json::to_string(&body)?);
    let bytes = hex::decode(digest.result_str()).expect("Hex string decoded");
    let digest_header = base64::encode(bytes);
    let date = SystemTime::now().into();

    let full_path = format!("https://{}{}", host, endpoint);

    let mut string = String::new();
    string.push_str(&format!("(request-target): get {}\n", endpoint));
    string.push_str(&format!("host: {}\n", host));
    string.push_str(&format!(
        "client-host: {}\n",
        "cs3099user-a9.host.cs.st-andrews.ac.uk"
    ));

    if let Some(u) = &uid {
        string.push_str(&format!("user-id: {}\n", &u));
    }
    string.push_str(&format!("date: {}\n", date));
    string.push_str(&format!("digest: SHA-512={}", digest_header));

    // create request to be signed (for testing purposes!)
    let req = rq_ctor(&awc::Client::new(), full_path)
        .header("User-Agent", "Actix Web")
        .header("Host", host)
        .header("Client-Host", "cs3099user-a9.host.cs.st-andrews.ac.uk")
        .header("Digest", ["sha-512=", &digest_header].join(""))
        .set(actix_web::http::header::Date(date));

    // Obtain private key from file and sign string
    let pkey = PKey::private_key_from_pem(&fs::read("fed_auth.pem").expect("reading key"))
        .expect("Getting private key.");
    let mut signer = Signer::new(MessageDigest::sha512(), &pkey)?;
    signer.set_rsa_padding(Padding::PKCS1)?;
    signer.update(string.as_bytes())?;

    // Base64 encode string
    let signature = signer.sign_to_vec()?;
    let encoded_sign = base64::encode(signature);

    // Append header to request
    let header_str = match &uid {
        Some(_) => "(request-target) host client-host user-id date digest",
        None => "(request-target) host client-host date digest",
    };

    let str_header = format!(
        "keyId=\"global\",algorithm=\"rsa-sha512\",headers=\"{}\",signature=\"{}\"",
        header_str, encoded_sign
    );

    let new_req = match &uid {
        Some(t) => req
            .header("User-ID", t.clone())
            .header("Signature", str_header),
        None => req.header("Signature", str_header),
    };

    // send request
    Ok(new_req.send())
}

pub async fn verify_federated_request<T>(
    request: HttpRequest,
    // need_user_id: bool,
    body: T,
) -> Result<bool, RouteError>
where
    T: Serialize,
{
    // Verify signature
    // get host from request
    let headers = request.headers();
    let host = format!("{:?}", headers.get("Host").ok_or("Missing Host Header."));
    let key_path = format!("{}{}{}", "https://", host, "/fed/key");

    // construct and send GET request to host/fed/key
    let date = SystemTime::now().into();
    let key_req = awc::Client::new()
        .get(key_path)
        .header("User-Agent", "Actix Web")
        .header("Host", host)
        .header("Client-Host", "cs3099user-a9.host.cs.st-andrews.ac.uk")
        .set(actix_web::http::header::Date(date))
        .send()
        .await
        .unwrap()
        .body()
        .await?;

    // using body of response, remove trailing lines from public key
    let pkey = PKey::public_key_from_pem(&key_req).expect("Error decoding public key.");

    // @TODO: generate the signature string :(
    let sign_str = "to-do".to_string();
    // @TODO: obtain base64 signature from header Signature (some string pattern matching?)
    let signature = "get signature!".to_string();
    // use openssl::Verifier with PCKS#1 to verify signature (and passing constructed string)
    let mut verifier = Verifier::new(MessageDigest::sha256(), &pkey)?;
    verifier.set_rsa_padding(Padding::PKCS1)?;
    verifier.update(sign_str.as_bytes())?;
    assert!(verifier.verify(signature.as_bytes())?);


    // Verify digest header
    let mut digest = Sha512::new();
    // hash body of request
    digest.input_str(&serde_json::to_string(&body)?);
    // encode output of hash
    let bytes = hex::decode(digest.result_str()).expect("Hex string decoded");
    let _digest_header = ["sha-512=", &base64::encode(bytes)].join("");
    // match digest header from request with above output

    Ok(true)
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
