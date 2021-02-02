use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BadResponse {
    title: String,
    message: String,
}

#[derive(thiserror::Error, Debug)]
pub enum RouteError {
    #[error("missing Client-Host")]
    MissingClientHost,
    #[error("missing User-ID")]
    MissingUserID,
}

impl ResponseError for RouteError {
    fn status_code(&self) -> StatusCode {
        match self {
            RouteError::MissingClientHost => StatusCode::BAD_REQUEST,
            RouteError::MissingUserID => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let title_message = match self {
            RouteError::MissingClientHost => "missing Client-Host".to_string(),
            RouteError::MissingUserID => "missing User-ID".to_string(),
        };

        HttpResponse::BadRequest().json(BadResponse {
            title: title_message.clone(),
            message: title_message,
        })
    }
}
