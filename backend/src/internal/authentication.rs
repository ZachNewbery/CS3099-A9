use jsonwebtoken::TokenData;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TODO: Write some static code for the token

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
    pub username: String,
    pub session: String,
}

impl Token {
    pub fn generate_token() -> String {
        todo!("use JWT + secret to generate token")
    }

    pub fn decode_token() -> jsonwebtoken::errors::Result<TokenData<Token>> {
        todo!("Use JWT + Secret to decode token")
    }
}
