
pub mod account;
use crate::error::AppError;
use actix_web::HttpResponse;
pub type HandlerResult = Result<HttpResponse, AppError>;
