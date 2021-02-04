use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct BadResponse {
    title: String,
    message: String,
}

// Errors that may be encountered during the processing of a route.
#[derive(thiserror::Error, Debug)]
pub enum RouteError {
    #[error("missing Client-Host")]
    MissingClientHost,
    #[error("missing User-ID")]
    MissingUserID,
    #[error("database error")]
    Diesel(#[from] diesel::result::Error),
    #[error("not found")]
    NotFound,
}

impl ResponseError for RouteError {
    fn status_code(&self) -> StatusCode {
        match self {
            RouteError::MissingClientHost => StatusCode::BAD_REQUEST,
            RouteError::MissingUserID => StatusCode::BAD_REQUEST,
            RouteError::Diesel(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::NotFound => StatusCode::NOT_FOUND
        }
    }

    fn error_response(&self) -> HttpResponse {
        let title_message = match self {
            RouteError::MissingClientHost => "missing Client-Host".to_string(),
            RouteError::MissingUserID => "missing User-ID".to_string(),
            RouteError::Diesel(e) => {
                eprintln!("{}", e);
                "internal database error".to_string()
            }
            RouteError::NotFound => {"not found".to_string()}
        };



        match self {
            RouteError::MissingClientHost => {
                HttpResponse::BadRequest()
            }
            RouteError::MissingUserID => {
                HttpResponse::BadRequest()
            }
            RouteError::Diesel(_) => {
                HttpResponse::InternalServerError()
            }
            RouteError::NotFound => {
                HttpResponse::NotFound()
            }
        }
        .json(BadResponse {
            title: title_message.clone(),
            message: title_message,
        })
    }
}
