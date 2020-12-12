use crate::core::QldbProcessor;
use crate::domain::TransactionType;
use crate::error::{AppError, ErrorType};
use bigdecimal::BigDecimal;

pub struct TransferService {
    processor: QldbProcessor,
}

impl TransferService {
    pub fn new(processor: QldbProcessor) -> TransferService {
        TransferService { processor }
    }

    pub async fn transfer(
        &self,
        sender_account_number: String,
        recipient_account_number: String,
        amount: BigDecimal,
    ) -> Result<String, AppError> {
        let message = self
            .processor
            .transfer(
                sender_account_number,
                recipient_account_number,
                amount.clone(),
            )
            .await?;
        if message == "INSUFFICIENT_BALANCE" {
            Err(AppError::from_type(ErrorType::InsufficientBalance))
        } else {
            Ok(message)
        }
    }

    pub async fn credit(
        &self,
        account_number: String,
        amount: BigDecimal,
    ) -> Result<String, AppError> {
        let message = self
            .processor
            .debit_credit(
                account_number.clone(),
                amount.clone(),
                TransactionType::CREDIT,
            )
            .await?;
        info!("Successfully credited ${} to {}", amount, account_number);
        Ok(message)
    }

    pub async fn debit(
        &self,
        account_number: String,
        amount: BigDecimal,
    ) -> Result<String, AppError> {
        let message = self
            .processor
            .debit_credit(account_number, amount.clone(), TransactionType::DEBIT)
            .await?;
        if message == "INSUFFICIENT_BALANCE" {
            Err(AppError::from_type(ErrorType::InsufficientBalance))
        } else {
            Ok(message)
        }
    }
}
