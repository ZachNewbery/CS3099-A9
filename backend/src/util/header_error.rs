use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BadResponse {
    title: String,
    message: String,
}

#[derive(thiserror::Error, Debug)]
pub enum HeaderError {
    #[error("missing Client-Host")]
    MissingClientHost,
    #[error("missing User-ID")]
    MissingUserID,
}

impl ResponseError for HeaderError {
    fn status_code(&self) -> StatusCode {
        match self {
            HeaderError::MissingClientHost => StatusCode::BAD_REQUEST,
            HeaderError::MissingUserID => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let title_message = match self {
            HeaderError::MissingClientHost => "missing Client-Host".to_string(),
            HeaderError::MissingUserID => "missing User-ID".to_string(),
        };

        HttpResponse::BadRequest().json(BadResponse {
            title: title_message.clone(),
            message: title_message,
        })
    }
}
