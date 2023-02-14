use core::fmt;

use actix_web::{error::ResponseError, http::StatusCode, HttpResponse};
use serde::Serialize;
use validator::ValidationErrors;

#[derive(Debug)]
pub enum AppErrorType {
    ValidationError,
}

#[derive(Debug)]
pub struct AppError {
    pub cause: Option<String>,
    pub message: Option<String>,
    pub error_type: AppErrorType,
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}

impl AppError {
    // we are handling the none. function name should match field name
    fn message(&self) -> String {
        match &*self {
            // Error message is found then clone otherwise default message
            AppError {
                cause: _,
                message: Some(message),
                error_type: _,
            } => message.clone(),
            AppError {
                cause: Some(cause),
                message: None,
                error_type: AppErrorType::ValidationError,
            } => cause.clone(),
            _ => "An unexpected error has occured".to_string(),
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl ResponseError for AppError {
    //error_response and status_code are the provided methods for ResponseError Trait

    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::ValidationError => StatusCode::LENGTH_REQUIRED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}

impl From<ValidationErrors> for AppError {
    fn from(error: ValidationErrors) -> AppError {
        AppError {
            message: None,
            cause: Some(error.to_string()),
            error_type: AppErrorType::ValidationError,
        }
    }
}

impl fmt::Display for AppErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for AppErrorType {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).finish()
    }
}
