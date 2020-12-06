
pub mod account;
pub mod transaction;
use crate::error::AppError;
use actix_web::HttpResponse;
pub type HandlerResult = Result<HttpResponse, AppError>;
