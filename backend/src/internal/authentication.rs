use chrono::Utc;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TODO: Update pipeline to generate "random" bytes
pub static JWT_SECRET_KEY: [u8; 16] = *include_bytes!("../../jwt_secret.key");

// Timeout of one week in seconds
const TIMEOUT: i64 = 60 * 60 * 24 * 7;

pub fn generate_session() -> String {
    Uuid::new_v4().to_simple().to_string()
}

// pub fn validate_token() -> jsonwebtoken::errors::Result<TokenData<Token>> {
//     unimplemented!()
// }

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub issued_at: i64,
    pub expiration: i64,
    pub user_id: u64,
    pub session: String,
}

impl Token {
    pub fn new(user_id: u64, session: &str) -> Self {
        Self {
            issued_at: Utc::now().timestamp(),
            expiration: Utc::now().timestamp() + TIMEOUT,
            user_id,
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

    pub fn decode_token(token: &str) -> jsonwebtoken::errors::Result<TokenData<Self>> {
        jsonwebtoken::decode(
            token,
            &DecodingKey::from_secret(&JWT_SECRET_KEY),
            &Validation::default(),
        )
    }
}
