//! Error mapping to reduce panicking
use actix_web::error::BlockingError;
use actix_web::http::header::ToStrError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use diesel::result::Error;
use serde::{Deserialize, Serialize};

/// Struct to represent a bad response
#[derive(Serialize, Deserialize)]
pub struct BadResponse {
    /// Title of the error
    title: String,
    /// Error message
    message: String,
}

/// Errors that may be encountered during the processing of a route
#[derive(thiserror::Error, Debug)]
pub enum RouteError {
    /// Missing Client-Host Header error
    #[error("missing Client-Host")]
    MissingClientHost,
    /// Invalid Client-Host Header error
    #[error("bad Client-Host")]
    BadClientHost,
    /// Invalid Public Key error
    #[error("bad Public Key")]
    BadKey,
    /// Missing User-ID Header error
    #[error("missing User-ID")]
    MissingUserId,
    /// Missing Date Header error
    #[error("missing Date")]
    MissingDate,
    /// Missing Signature Header error
    #[error("missing Signature")]
    MissingSignature,
    /// Invalid Digest Header error
    #[error("bad Digest")]
    BadDigest,
    /// Invalid Signature Header error
    #[error("bad Signature Header")]
    BadSignHeader,
    /// Diesel Errors
    #[error(transparent)]
    Diesel(diesel::result::Error), // no "from" proc-macro here because we define it ourselves
    /// Database retrieval error (NotFound)
    #[error("item not found in database")]
    NotFound,
    /// Actix-web Error
    #[error("actix internal")]
    ActixInternal,
    /// Actix-web Payload error
    #[error(transparent)]
    Payload(#[from] actix_web::error::PayloadError),
    /// External service error
    #[error("external service error")]
    ExternalService,
    /// UUID parsing error
    #[error(transparent)]
    UuidParse(#[from] uuid::Error),
    /// Header parsing error
    #[error(transparent)]
    HeaderParse(#[from] ToStrError),
    /// Hex parsing error
    #[error(transparent)]
    Hex(#[from] hex::FromHexError),
    /// Json serialization error
    #[error(transparent)]
    JsonSerde(#[from] serde_json::Error),
    /// Json URL encoding error
    #[error(transparent)]
    JsonSerdeUrl(#[from] serde_urlencoded::ser::Error),
    /// OpenSSL error
    #[error(transparent)]
    OpenSsl(#[from] openssl::error::ErrorStack),
    /// Timeout error
    #[error("timed out")]
    Cancelled,
    /// Unsupported Content Type error
    #[error("unsupported content type")]
    UnsupportedContentType,
    /// Invalid post content error
    #[error("bad post content")]
    BadPostContent,
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
            RouteError::BadKey => StatusCode::BAD_REQUEST,
            RouteError::BadClientHost => StatusCode::BAD_REQUEST,
            RouteError::BadDigest => StatusCode::BAD_REQUEST,
            RouteError::BadSignHeader => StatusCode::BAD_REQUEST,
            RouteError::Diesel(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::NotFound => StatusCode::NOT_FOUND,
            RouteError::UuidParse(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::HeaderParse(_) => StatusCode::BAD_REQUEST,
            RouteError::Hex(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::JsonSerde(_) => StatusCode::BAD_REQUEST,
            RouteError::JsonSerdeUrl(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::OpenSsl(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::ActixInternal => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::Payload(_) => StatusCode::INTERNAL_SERVER_ERROR,
            RouteError::ExternalService => StatusCode::BAD_GATEWAY,
            RouteError::Cancelled => StatusCode::REQUEST_TIMEOUT,
            RouteError::UnsupportedContentType => StatusCode::BAD_REQUEST,
            RouteError::BadPostContent => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let title_message = match self {
            RouteError::MissingClientHost => "Missing Client-Host".to_string(),
            RouteError::MissingUserId => "Missing User-ID".to_string(),
            RouteError::MissingDate => "Missing Date".to_string(),
            RouteError::MissingSignature => "Missing Signature".to_string(),
            RouteError::BadKey => "Invalid public key".to_string(),
            RouteError::BadClientHost => "Bad Client-Host header".to_string(),
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
            RouteError::JsonSerdeUrl(_) => "Could not parse URL query.".to_string(),
            RouteError::OpenSsl(_) => {
                "OpenSSL error: Invalid PEM Public Key received from /fed/key".to_string()
            }
            RouteError::ActixInternal => "Invalid body of request".to_string(),
            RouteError::Payload(_) => "Could not obtain request body for validation".to_string(),
            RouteError::ExternalService => {
                "Could not connect to external host when requesting key".to_string()
            }
            RouteError::Cancelled => "Task timed out".to_string(),
            RouteError::UnsupportedContentType => "unsupported content typed used".to_string(),
            RouteError::BadPostContent => "invalid content in post".to_string(),
        };

        match self {
            RouteError::MissingClientHost => HttpResponse::BadRequest(),
            RouteError::MissingUserId => HttpResponse::BadRequest(),
            RouteError::MissingDate => HttpResponse::BadRequest(),
            RouteError::MissingSignature => HttpResponse::BadRequest(),
            RouteError::BadKey => HttpResponse::BadRequest(),
            RouteError::BadClientHost => HttpResponse::BadRequest(),
            RouteError::BadDigest => HttpResponse::BadRequest(),
            RouteError::BadSignHeader => HttpResponse::BadRequest(),
            RouteError::Diesel(_) => HttpResponse::InternalServerError(),
            RouteError::NotFound => HttpResponse::NotFound(),
            RouteError::UuidParse(_) => HttpResponse::InternalServerError(),
            RouteError::HeaderParse(_) => HttpResponse::BadRequest(),
            RouteError::Hex(_) => HttpResponse::InternalServerError(),
            RouteError::JsonSerde(_) => HttpResponse::BadRequest(),
            RouteError::JsonSerdeUrl(_) => HttpResponse::InternalServerError(),
            RouteError::OpenSsl(_) => HttpResponse::InternalServerError(),
            RouteError::ActixInternal => HttpResponse::InternalServerError(),
            RouteError::Payload(_) => HttpResponse::InternalServerError(),
            RouteError::ExternalService => HttpResponse::BadGateway(),
            RouteError::Cancelled => HttpResponse::RequestTimeout(),
            RouteError::UnsupportedContentType => HttpResponse::BadRequest(),
            RouteError::BadPostContent => HttpResponse::BadRequest(),
        }
        .json(BadResponse {
            title: title_message.clone(),
            message: title_message,
        })
    }
}

impl From<BlockingError<RouteError>> for RouteError {
    fn from(val: BlockingError<RouteError>) -> Self {
        match val {
            BlockingError::Error(e) => e,
            BlockingError::Canceled => RouteError::Cancelled,
        }
    }
}
