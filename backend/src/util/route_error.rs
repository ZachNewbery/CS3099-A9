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
    #[error("missing Date")]
    MissingDate,
    #[error("missing Signature")]
    MissingSignature,
    #[error("bad Digest")]
    BadDigest,
    #[error("bad Signature Header")]
    BadSignHeader,
    #[error(transparent)]
    Diesel(diesel::result::Error), // no "from" proc-macro here because we define it ourselves
    #[error("item not found in database")]
    NotFound,
    #[error("actix internal")]
    ActixInternal,
    #[error(transparent)]
    Payload(#[from] actix_web::error::PayloadError),
    #[error("external service error")]
    ExternalService,
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
            RouteError::MissingDate => StatusCode::BAD_REQUEST,
            RouteError::MissingSignature => StatusCode::BAD_REQUEST,
            RouteError::BadDigest => StatusCode::BAD_REQUEST,
            RouteError::BadSignHeader => StatusCode::BAD_REQUEST,
            RouteError::Diesel(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::NotFound => StatusCode::NOT_FOUND,
            RouteError::UuidParse(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::HeaderParse(_) => StatusCode::BAD_REQUEST,
            RouteError::Hex(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::JsonSerde(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::OpenSsl(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::ActixInternal => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::Payload(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::ExternalService => StatusCode::BAD_GATEWAY,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let title_message = match self {
            RouteError::MissingClientHost => "Missing Client-Host".to_string(),
            RouteError::MissingUserId => "Missing User-ID".to_string(),
            RouteError::MissingDate => "Missing Date".to_string(),
            RouteError::MissingSignature => "Missing Signature".to_string(),
            RouteError::BadDigest => "Invalid Digest".to_string(),
            RouteError::BadSignHeader => "Invalid Signature header".to_string(),
            RouteError::Diesel(e) => {
                eprintln!("{}", e);
                "Internal database error".to_string()
            }
            RouteError::NotFound => "Not Found".to_string(),
            RouteError::UuidParse(e) => {
                eprintln!("{}", e);
                "Internal server error".to_string()
            }
            RouteError::HeaderParse(_) => "Invalid Headers".to_string(),
            RouteError::Hex(_) => "Hex could not be decoded.".to_string(),
            RouteError::JsonSerde(_) => "JSON could not be parsed.".to_string(),
            RouteError::OpenSsl(_) => {
                "OpenSSL error: Invalid PEM Public Key received from /fed/key".to_string()
            }
            RouteError::ActixInternal => "Invalid body of request".to_string(),
            RouteError::Payload(_) => {
                "Could not obtain request body for validation (possibly from /fed/key)".to_string()
            }
            RouteError::ExternalService => {
                "Could not connect to external host when requesting key".to_string()
            }
        };

        match self {
            RouteError::MissingClientHost => HttpResponse::BadRequest(),
            RouteError::MissingUserId => HttpResponse::BadRequest(),
            RouteError::MissingDate => HttpResponse::BadRequest(),
            RouteError::MissingSignature => HttpResponse::BadRequest(),
            RouteError::BadDigest => HttpResponse::BadRequest(),
            RouteError::BadSignHeader => HttpResponse::BadRequest(),
            RouteError::Diesel(_) => HttpResponse::InternalServerError(),
            RouteError::NotFound => HttpResponse::NotFound(),
            RouteError::UuidParse(_) => HttpResponse::InternalServerError(),
            RouteError::HeaderParse(_) => HttpResponse::BadRequest(),
            RouteError::Hex(_) => HttpResponse::InternalServerError(),
            RouteError::JsonSerde(_) => HttpResponse::InternalServerError(),
            RouteError::OpenSsl(_) => HttpResponse::InternalServerError(),
            RouteError::ActixInternal => HttpResponse::InternalServerError(),
            RouteError::Payload(_) => HttpResponse::InternalServerError(),
            RouteError::ExternalService => HttpResponse::BadGateway(),
        }
        .json(BadResponse {
            title: title_message.clone(),
            message: title_message,
        })
    }
}
