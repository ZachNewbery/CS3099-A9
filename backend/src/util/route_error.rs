use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error;
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
    #[error(transparent)]
    Diesel(diesel::result::Error), // no "from" proc-macro here because we define it ourselves
    #[error("item not found in database")]
    NotFound,
    #[error(transparent)]
    UuidParse(#[from] uuid::Error),
}

impl From<diesel::result::Error> for RouteError {
    fn from(value: Error) -> Self {
        match value {
            Error::NotFound => Self::NotFound,
            _ => Self::Diesel(value),
        }
    }
}

impl ResponseError for RouteError {
    fn status_code(&self) -> StatusCode {
        match self {
            RouteError::MissingClientHost => StatusCode::BAD_REQUEST,
            RouteError::MissingUserID => StatusCode::BAD_REQUEST,
            RouteError::Diesel(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::NotFound => StatusCode::NOT_FOUND,
            RouteError::UuidParse(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
            RouteError::NotFound => "not found".to_string(),
            RouteError::UuidParse(e) => {
                eprintln!("{}", e);
                "internal server error".to_string()
            }
        };

        match self {
            RouteError::MissingClientHost => HttpResponse::BadRequest(),
            RouteError::MissingUserID => HttpResponse::BadRequest(),
            RouteError::Diesel(_) => HttpResponse::InternalServerError(),
            RouteError::NotFound => HttpResponse::NotFound(),
            RouteError::UuidParse(_) => HttpResponse::InternalServerError(),
        }
        .json(BadResponse {
            title: title_message.clone(),
            message: title_message,
        })
    }
}
