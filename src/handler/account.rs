use super::HandlerResult;
use crate::domain::{AppState, NewAccount, Response};
use crate::service::AccountService;
use actix_web::{
    web::{self, Json},
    HttpResponse,
};

pub async fn get_accounts(app_state: web::Data<AppState>) -> HandlerResult {
    let account_service = AccountService::new(app_state.processor.clone());
    let accounts = account_service.find_accounts().await?;
    Ok(HttpResponse::Ok().json(accounts))
}

pub async fn get_account(app_state: web::Data<AppState>, path: web::Path<String>) -> HandlerResult {
    let account_number = path.into_inner();
    let account_service = AccountService::new(app_state.processor.clone());
    let account = account_service.find_account(account_number).await?;
    Ok(HttpResponse::Ok().json(account))
}

pub async fn delete_account(
    app_state: web::Data<AppState>,
    path: web::Path<String>,
) -> HandlerResult {
    let account_number = path.into_inner();
    let account_service = AccountService::new(app_state.processor.clone());
    account_service.delete_account(account_number).await?;
    Ok(HttpResponse::Ok().json(Response::new("Successfully deleted accoun")))
}
pub async fn create_account(
    app_state: web::Data<AppState>,
    new_account: Json<NewAccount>,
) -> HandlerResult {
    let account_service = AccountService::new(app_state.processor.clone());
    let (_, account) = account_service
        .create_account(new_account.into_inner())
        .await?;
    Ok(HttpResponse::Created().json(account))
}
