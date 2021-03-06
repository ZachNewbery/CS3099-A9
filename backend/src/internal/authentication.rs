//! Authentication handling functions
use actix_web::http::header::Header as ActixHeader;
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_httpauth::headers::authorization::{Authorization, Bearer};

use chrono::Utc;
use futures::StreamExt;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use openssl::hash::*;
use openssl::pkey::*;
use openssl::rsa::Padding;
use openssl::sign::*;
use sha2::*;
use std::time::Duration;

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
use awc::{http, ClientRequest, SendClientRequest};

/// JWT Secret Key used to generate Tokens.
pub static JWT_SECRET_KEY: [u8; 16] = *include_bytes!("../../jwt_secret.key");

/// Timeout of one week in seconds
const TIMEOUT: i64 = 60 * 60 * 24 * 7;

/// Max request payload size of 256kBs
const MAX_SIZE: usize = 262_144;

/// Returns a new UUID identifying a new session
pub fn generate_session() -> String {
    Uuid::new_v4().to_simple().to_string()
}

/// Struct representing a JWT Authentication Token
#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    /// Issue time of Token
    #[serde(rename = "iat")]
    pub issued_at: i64,
    /// Expiry time of Token
    #[serde(rename = "exp")]
    pub expiration: i64,
    /// Internal id of the row in LocalUsers the Token belongs to
    pub id: u64,
    /// Session the Token validates
    pub session: String,
}

impl Token {
    /// Returns a new session for use to generate a Token
    pub fn new(id: u64, session: &str) -> Self {
        Self {
            issued_at: Utc::now().timestamp(),
            expiration: Utc::now().timestamp() + TIMEOUT,
            id,
            session: session.to_string(),
        }
    }

    /// Generates a JWT Token for a session using the JWT secret key
    pub fn generate_token(&self) -> jsonwebtoken::errors::Result<String> {
        jsonwebtoken::encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(&JWT_SECRET_KEY),
        )
    }

    /// Decodes a JWT Token to ensure validity
    pub fn decode_token(token: &str) -> jsonwebtoken::errors::Result<TokenData<Token>> {
        jsonwebtoken::decode(
            token,
            &DecodingKey::from_secret(&JWT_SECRET_KEY),
            &Validation::default(),
        )
    }
}

/// Authenticates a LocalUser upon login to ensure session is valid
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

/// Wrapper function to send a federated request with the supergroup authentication
pub fn make_federated_request<T, U>(
    rq_ctor: fn(&awc::Client, url: String) -> ClientRequest,
    host: String,
    endpoint: String,
    body: T,
    uid: Option<String>,
    query: Option<U>,
) -> Result<SendClientRequest, RouteError>
where
    T: Serialize,
    U: Serialize,
{
    // hash body of HTTP request
    let body_string = serde_json::to_string(&body)?;
    let digest_header = &base64::encode(Sha512::digest(body_string.as_bytes()));
    let date = SystemTime::now().into();

    let full_path = format!("https://{}{}", host, endpoint);

    // create request to be signed
    let mut req = rq_ctor(&awc::Client::new(), full_path)
        .header("User-Agent", "Actix Web")
        .header("Host", host.clone())
        .header("Client-Host", "cs3099user-a9.host.cs.st-andrews.ac.uk")
        .header("Digest", ["sha-512=", &digest_header].join(""))
        .set(actix_web::http::header::Date(date));

    if !body_string.is_empty() {
        req = req.header(http::header::CONTENT_TYPE, "application/json");
    }

    if let Some(q) = query {
        req = req.query(&q)?;
    }

    let string = {
        let mut s = String::new();

        s.push_str(&format!(
            "(request-target): {} {}\n",
            req.get_method().as_str().to_lowercase(),
            endpoint
        ));

        s.push_str(&format!("host: {}\n", host));
        s.push_str(&format!(
            "client-host: {}\n",
            "cs3099user-a9.host.cs.st-andrews.ac.uk"
        ));

        if let Some(u) = &uid {
            s.push_str(&format!("user-id: {}\n", &u));
        }
        s.push_str(&format!("date: {}\n", date));
        s.push_str(&format!("digest: SHA-512={}", digest_header));

        s
    };

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
    if body_string.is_empty() {
        Ok(new_req.send())
    } else {
        Ok(new_req.send_json(&body))
    }
}

/// Wrapper method to verify an incoming request using the supergroup protocol authentication
pub async fn verify_federated_request(
    request: HttpRequest,
    mut payload: web::Payload,
) -> Result<web::Bytes, RouteError> {
    // get host from request
    let headers = request.headers();
    let client_host = headers
        .get("Client-Host")
        .ok_or(RouteError::MissingClientHost)?
        .to_str()?;

    if client_host == "localhost" {
        Err(RouteError::BadClientHost)
    } else {
        // hash body of request to create digest
        let mut body = web::BytesMut::new();
        while let Some(chunk) = payload.next().await {
            let chunk = chunk?;
            if (body.len() + chunk.len()) > MAX_SIZE {
                return Err(RouteError::ActixInternal);
            }
            body.extend_from_slice(&chunk);
        }
        let digest_header = &base64::encode(Sha512::digest(&body));

        // construct and send GET request to host/fed/key
        let key_path = format!("https://{}/fed/key", client_host);
        let connector = awc::Connector::new()
            .timeout(Duration::from_secs(3))
            .finish();

        let client = awc::Client::builder()
            .connector(connector)
            .timeout(Duration::from_secs(5))
            .finish();

        let mut key_req = client
            .get(key_path)
            .header("User-Agent", "Actix Web")
            .header("Host", client_host.to_string())
            .header("Client-Host", "cs3099user-a9.host.cs.st-andrews.ac.uk")
            .send()
            .await
            .map_err(|_| RouteError::BadKey)?;

        // using body of response, get public key
        let key_req = key_req.body().await?;
        let pkey = PKey::public_key_from_pem(&key_req)?;

        // generate expected signature string
        let mut string = String::new();
        string.push_str(&format!(
            "(request-target): {} {}\n",
            request.method().as_str().to_lowercase(),
            request.path()
        ));
        string.push_str(&format!(
            "host: {}\n",
            "cs3099user-a9.host.cs.st-andrews.ac.uk"
        ));
        string.push_str(&format!("client-host: {}\n", client_host));
        if let Some(userid) = headers.get("User-ID") {
            let uid = userid.to_str()?;
            string.push_str(&format!("user-id: {}\n", &uid));
        }
        string.push_str(&format!(
            "date: {}\n",
            headers
                .get("Date")
                .ok_or(RouteError::MissingDate)?
                .to_str()?
        ));
        string.push_str(&format!("digest: SHA-512={}", digest_header));

        //obtain base64 signature from header Signature and match it to the expected string
        let sign_header = headers
            .get("Signature")
            .ok_or(RouteError::MissingSignature)?
            .to_str()?;

        let mut split = sign_header.split(",signature=").collect::<Vec<_>>();

        let _ = split.pop().ok_or(RouteError::BadSignHeader)?;

        let signature = split.pop().ok_or(RouteError::BadSignHeader)?;

        // verify signature with expected string
        let mut verifier = Verifier::new(MessageDigest::sha256(), &pkey)?;
        verifier.set_rsa_padding(Padding::PKCS1)?;
        verifier.update(string.as_bytes())?;
        verifier.verify(signature.as_bytes())?;

        // verify digest header through matching
        let exp_digest = ["sha-512=", digest_header].join("");
        let given_digest = headers
            .get("Digest")
            .ok_or(RouteError::BadDigest)?
            .to_str()?;
        if exp_digest != given_digest {
            println!("Could not match digest. Expected {}", exp_digest);
            println!("Given digest {}", given_digest);
            Err(RouteError::BadDigest)
        } else {
            Ok(body.freeze())
        }
    }
}
