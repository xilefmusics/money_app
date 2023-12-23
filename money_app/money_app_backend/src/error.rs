use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use csv;
use fancy_surreal;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Database(String),
    Unauthorized(String),
    TypeConvertError(String),
    Filesystem(String),
    NotFound(String),
    Import(String),
    Other(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Database(message) => write!(f, "DatabaseError ({})", message),
            Self::Unauthorized(message) => write!(f, "UnauthorizedError ({})", message),
            Self::TypeConvertError(message) => write!(f, "TypeConvertError ({})", message),
            Self::Filesystem(message) => write!(f, "FilesystemError ({})", message),
            Self::NotFound(message) => write!(f, "NotFoundError ({})", message),
            Self::Import(message) => write!(f, "OtherError ({})", message),
            Self::Other(message) => write!(f, "OtherError ({})", message),
        }
    }
}

impl error::Error for AppError {}

impl From<fancy_surreal::Error> for AppError {
    fn from(err: fancy_surreal::Error) -> Self {
        Self::Database(err.to_string())
    }
}

impl From<csv::Error> for AppError {
    fn from(err: csv::Error) -> Self {
        Self::Import(err.to_string())
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::TypeConvertError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Filesystem(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Import(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Other(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        log::error!("{}", self);
        match self {
            Self::Database(_) => {
                HttpResponse::build(self.status_code()).body("500 Internal Server Error")
            }
            Self::Unauthorized(_) => {
                HttpResponse::build(self.status_code()).body("401 Unauthorized")
            }
            Self::TypeConvertError(_) => {
                HttpResponse::build(self.status_code()).body("500 Internal Server Error")
            }
            Self::Filesystem(_) => {
                HttpResponse::build(self.status_code()).body("500 Internal Server Error")
            }
            Self::NotFound(_) => HttpResponse::build(self.status_code()).body("404 Not Found"),
            Self::Import(_) => {
                HttpResponse::build(self.status_code()).body("500 Internal Server Error")
            }
            Self::Other(_) => {
                HttpResponse::build(self.status_code()).body("500 Internal Server Error")
            }
        }
    }
}
