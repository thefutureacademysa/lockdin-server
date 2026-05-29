use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    UserAlreadyExists,
    DatabaseError(sqlx::Error),
    InternalServerError(String),
    TokenGenerationError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::UserAlreadyExists => write!(f, "User with this phone number already exists"),
            AppError::DatabaseError(err) => write!(f, "Database error: {}", err),
            AppError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
            AppError::TokenGenerationError(msg) => write!(f, "Token generation error: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::UserAlreadyExists => StatusCode::CONFLICT,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::TokenGenerationError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error_message = self.to_string();
        let response = ErrorResponse {
            error: error_message,
        };

        match self {
            AppError::UserAlreadyExists => {
                HttpResponse::Conflict().json(response)
            }
            AppError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(response)
            }
            AppError::InternalServerError(_) => {
                HttpResponse::InternalServerError().json(response)
            }
            AppError::TokenGenerationError(_) => {
                HttpResponse::InternalServerError().json(response)
            }
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err)
    }
}
