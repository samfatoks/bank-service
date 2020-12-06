use super::HandlerResult;
use crate::{Config, domain::TransactionType};
use crate::service::{AccountService, TransferService};
use crate::error::{AppError, ErrorType};
use crate::domain::NewTransaction;
use serde_json::json;
use actix_web::{
    web::{self, Json},
    HttpResponse,
};


pub async fn handle_transaction(config: web::Data<Config>, new_transaction: Json<NewTransaction>) -> HandlerResult {
    let ledger_name = &config.ledger_name;
    let transaction = new_transaction.into_inner();
    if transaction.transaction_type == TransactionType::TRANSFER && transaction.sender_account_number.clone().is_none(){
        return Err(AppError::new(Some("sender_account_number cannot be empty for transfer".to_string()), ErrorType::PayloadError));
    }

    let account_service = AccountService::new(ledger_name.clone()).await?;    
    let amount = transaction.amount;
    let recipient_account_number = transaction.recipient_account_number;
    account_service.find_account(recipient_account_number.clone()).await.map_err(|e| {
        match e.error_type {
            ErrorType::AccountNotFound(_) => {
                AppError::new(Some("Receipient account not found".to_string()), ErrorType::AccountNotFound(recipient_account_number.clone()))
            },
            _ => e
        }
    })?;

    let transfer_service = TransferService::new(ledger_name.clone()).await?;
    let message = match transaction.transaction_type {
        crate::domain::TransactionType::CREDIT => {
            transfer_service.credit(recipient_account_number.clone(), amount).await?
        }
        crate::domain::TransactionType::DEBIT => {
            transfer_service.debit(recipient_account_number.clone(), amount).await?
        }
        crate::domain::TransactionType::TRANSFER => {
            let sender_account_number = transaction.sender_account_number.unwrap();
            account_service.find_account(sender_account_number.clone()).await.map_err(|e| {
                match e.error_type {
                    ErrorType::AccountNotFound(_) => {
                        AppError::new(Some("Sender account not found".to_string()), ErrorType::AccountNotFound(sender_account_number.clone()))
                    },
                    _ => e
                }
            })?;
            transfer_service.transfer(sender_account_number, recipient_account_number, amount).await?
        }
    };


    Ok(HttpResponse::Ok().json(json!({"message": message})))
}

