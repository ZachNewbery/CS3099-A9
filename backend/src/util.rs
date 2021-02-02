use actix_web::{ResponseError, HttpResponse};
use actix_web::http::StatusCode;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct BadResponse {
    title: String,
    message: String
}

#[derive(thiserror::Error, Debug)]
pub enum RouteError {
    #[error("missing Client-Host")]
    MissingClientHost,
}

impl ResponseError for RouteError {
    fn status_code(&self) -> StatusCode {
        match self {
            RouteError::MissingClientHost => { StatusCode::BAD_REQUEST }
        }
    }

    fn error_response(&self) -> HttpResponse {
        match self {
            RouteError::MissingClientHost => {
                HttpResponse::BadRequest().json(BadResponse {
                    title: "missing Client-Host".to_string(),
                    message: "missing Client-Host".to_string()
                })
            }
        }
    }
}