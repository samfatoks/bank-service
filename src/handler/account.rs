use super::HandlerResult;
use crate::Config;
use crate::service::AccountService;
use crate::error::{AppError, ErrorType};
use crate::domain::NewAccount;
use serde_json::json;
use actix_web::{
    web::{self, Json},
    HttpResponse,
};

pub async fn get_accounts(config: web::Data<Config>) -> HandlerResult {
    let ledger_name = &config.ledger_name;
    let account_service = AccountService::new(ledger_name.clone()).await?;
    let accounts = account_service.find_accounts().await?;
    Ok(HttpResponse::Ok().json(accounts))
}

pub async fn get_account(config: web::Data<Config>, path: web::Path<String>) -> HandlerResult {
    let ledger_name = &config.ledger_name;
    let account_number = path.into_inner();
    let account_service = AccountService::new(ledger_name.clone()).await?;
    let account = account_service.find_account(account_number).await?;
    Ok(HttpResponse::Ok().json(account))
}

pub async fn delete_account(config: web::Data<Config>, path: web::Path<String>) -> HandlerResult {
    let ledger_name = &config.ledger_name;
    let account_number = path.into_inner();
    let account_service = AccountService::new(ledger_name.clone()).await?;
    account_service.delete_account(account_number).await?;
    Ok(HttpResponse::Ok().json(json!({"message": "Successfully deleted accoun"})))
}
pub async fn create_account(config: web::Data<Config>, new_account: Json<NewAccount>) -> HandlerResult {
    let ledger_name = &config.ledger_name;
    let account_service = AccountService::new(ledger_name.clone()).await?;

    let (_, account) = account_service.create_account(new_account.into_inner()).await?;
    Ok(HttpResponse::Created().json(account))
}
