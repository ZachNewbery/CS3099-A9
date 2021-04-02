use actix_web::http::header::ToStrError;
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
    // TODO: Refactor this to use string
    #[error("missing Client-Host")]
    MissingClientHost,
    #[error("missing User-ID")]
    MissingUserId,
    #[error("bad Digest")]
    BadDigest,
    #[error("bad Signature Header")]
    BadSignHeader,
    #[error(transparent)]
    Diesel(diesel::result::Error), // no "from" proc-macro here because we define it ourselves
    #[error("item not found in database")]
    NotFound,
    #[error(transparent)]
    ActixWeb(#[from] actix_web::error::PayloadError),
    #[error(transparent)]
    UuidParse(#[from] uuid::Error),
    #[error(transparent)]
    HeaderParse(#[from] ToStrError),
    #[error(transparent)]
    Hex(#[from] hex::FromHexError),
    #[error(transparent)]
    JsonSerde(#[from] serde_json::Error),
    #[error(transparent)]
    OpenSsl(#[from] openssl::error::ErrorStack),
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
            RouteError::MissingUserId => StatusCode::BAD_REQUEST,
            RouteError::BadDigest => StatusCode::BAD_REQUEST,
            RouteError::BadSignHeader => StatusCode::BAD_REQUEST,
            RouteError::Diesel(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::NotFound => StatusCode::NOT_FOUND,
            RouteError::UuidParse(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::HeaderParse(_) => StatusCode::BAD_REQUEST,
            RouteError::Hex(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::JsonSerde(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::OpenSsl(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::ActixWeb(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let title_message = match self {
            RouteError::MissingClientHost => "missing Client-Host".to_string(),
            RouteError::MissingUserId => "missing User-ID".to_string(),
            RouteError::BadDigest => "bad Digest".to_string(),
            RouteError::BadSignHeader => {
                "bad Signature header: is user-id required in headers= for this request?"
                    .to_string()
            }
            RouteError::Diesel(e) => {
                eprintln!("{}", e);
                "internal database error".to_string()
            }
            RouteError::NotFound => "not found".to_string(),
            RouteError::UuidParse(e) => {
                eprintln!("{}", e);
                "internal server error".to_string()
            }
            RouteError::HeaderParse(_) => "bad headers".to_string(),
            RouteError::Hex(_) => "could not decode hex".to_string(),
            RouteError::JsonSerde(_) => "could not parse json".to_string(),
            RouteError::OpenSsl(_) => "openssl error".to_string(),
            RouteError::ActixWeb(_) => "actix-web error".to_string(),
        };

        match self {
            RouteError::MissingClientHost => HttpResponse::BadRequest(),
            RouteError::MissingUserId => HttpResponse::BadRequest(),
            RouteError::BadDigest => HttpResponse::BadRequest(),
            RouteError::BadSignHeader => HttpResponse::BadRequest(),
            RouteError::Diesel(_) => HttpResponse::InternalServerError(),
            RouteError::NotFound => HttpResponse::NotFound(),
            RouteError::UuidParse(_) => HttpResponse::InternalServerError(),
            RouteError::HeaderParse(_) => HttpResponse::BadRequest(),
            RouteError::Hex(_) => HttpResponse::InternalServerError(),
            RouteError::JsonSerde(_) => HttpResponse::InternalServerError(),
            RouteError::OpenSsl(_) => HttpResponse::InternalServerError(),
            RouteError::ActixWeb(_) => HttpResponse::InternalServerError(),
        }
        .json(BadResponse {
            title: title_message.clone(),
            message: title_message,
        })
    }
}
