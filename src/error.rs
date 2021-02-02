use actix_web::{
    error::{JsonPayloadError, ResponseError},
    http::StatusCode,
    HttpResponse,
};
use ion_binary_rs::IonParserError;
use qldb::{QLDBError, QLDBExtractError};
use serde::Serialize;
use std::fmt;

#[derive(Debug)]
pub enum ErrorType {
    Custom(String),
    AccountError(String),
    IonError(IonParserError),
    QLDBError(QLDBError),
    QLDBExtractError(QLDBExtractError),
    InsufficientBalance,
    AccountNotFound(String),
    NoRowsAffected,
    PayloadError,
}

impl fmt::Display for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorType::Custom(message) => write!(f, "{}", message),
            ErrorType::AccountError(message) => write!(f, "{}", message),
            ErrorType::IonError(s) => write!(f, "Ion Parser Error: {}", s),
            ErrorType::QLDBError(s) => write!(f, "QLDB Error: {}", s),
            ErrorType::QLDBExtractError(s) => write!(f, "QLDB Extract Error: {}", s),
            ErrorType::InsufficientBalance => write!(f, "Insufficient balance in account"),
            ErrorType::AccountNotFound(s) => write!(f, "Account not found: {}", s),
            ErrorType::NoRowsAffected => write!(f, "No rows affected"),
            _ => write!(f, "Unable to process request"),
        }
    }
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub error_type: ErrorType,
}

impl AppError {
    fn message(&self) -> String {
        match &*self {
            AppError {
                message: Some(message),
                error_type: _,
            } => message.clone(),
            AppError {
                message: None,
                error_type: e,
            } => e.to_string(),
        }
    }
    fn error_type(&self) -> String {
        let error = match self.error_type {
            ErrorType::InsufficientBalance | ErrorType::AccountNotFound(_) => "Transaction Error",
            ErrorType::PayloadError => "Payload Error",
            ErrorType::AccountError(_) => "Account Error",
            _ => "Platform Error",
        };
        error.to_string()
    }
    pub fn new(message: Option<String>, error_type: ErrorType) -> Self {
        AppError {
            message,
            error_type,
        }
    }
    pub fn from_type(error_type: ErrorType) -> Self {
        AppError {
            message: None,
            error_type,
        }
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl From<std::str::Utf8Error> for AppError {
    fn from(err: std::str::Utf8Error) -> Self {
        AppError::new(None, ErrorType::Custom(err.to_string()))
    }
}

impl From<IonParserError> for AppError {
    fn from(err: IonParserError) -> Self {
        AppError::new(None, ErrorType::IonError(err))
    }
}

impl From<QLDBError> for AppError {
    fn from(err: QLDBError) -> Self {
        AppError::new(None, ErrorType::QLDBError(err))
    }
}

impl From<QLDBExtractError> for AppError {
    fn from(err: QLDBExtractError) -> Self {
        AppError::new(None, ErrorType::QLDBExtractError(err))
    }
}

impl From<JsonPayloadError> for AppError {
    fn from(error: JsonPayloadError) -> Self {
        match error {
            JsonPayloadError::Deserialize(e) => AppError {
                message: Some(e.to_string()),
                error_type: ErrorType::PayloadError,
            },
            JsonPayloadError::Overflow => AppError {
                message: Some("Payload too large".to_string()),
                error_type: ErrorType::PayloadError,
            },
            _ => AppError {
                message: Some("Unable to parse json playload".to_string()),
                error_type: ErrorType::PayloadError,
            },
        }
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub message: String,
    pub error: String,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            ErrorType::InsufficientBalance => StatusCode::BAD_REQUEST,
            ErrorType::PayloadError => StatusCode::BAD_REQUEST,
            ErrorType::AccountNotFound(_) => StatusCode::NOT_FOUND,
            ErrorType::AccountError(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            message: self.message(),
            error: self.error_type(),
        })
    }
}
