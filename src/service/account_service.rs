use crate::{Config, core::QldbProcessor};
use crate::domain::{Account, NewAccount, TransactionType};
use bigdecimal::BigDecimal;
use std::convert::TryInto;
use crate::error::{AppError, ErrorType};

pub struct AccountService {
    processor: QldbProcessor
}

impl AccountService {
    pub async fn new(ledger_name: String) -> Result<AccountService, AppError>  {
        let processor = QldbProcessor::new(ledger_name).await?;
        Ok(AccountService { processor })
    }

    pub async fn create_account(&self, new_account: NewAccount) -> Result<(String, Account), AppError>{
        let account: Account = new_account.into();
        let document_id = self.processor.insert(&account).await?;
        Ok((document_id, account))
    }

    pub async fn transfer(&self, src_account_number: String, dst_account_number: String, amount: BigDecimal) -> Result<String, AppError> {
        let message = self.processor.transfer(src_account_number, dst_account_number, amount.clone()).await?;
        if message == "INSUFFICIENT_BALANCE" {
            Err(AppError::from_type(ErrorType::InsufficientBalance))
        } else {
            Ok(message)
        }
    }

    pub async fn credit(&self, account_number: String, amount: BigDecimal) -> Result<String, AppError> {
        let message = self.processor.debit_credit(account_number.clone(), amount.clone(), TransactionType::CREDIT).await?;
        info!("Successfully credited ${} to {}", amount, account_number);
        Ok(message)
    }

    pub async fn debit(&self, account_number: String, amount: BigDecimal) -> Result<String, AppError> {
        let message = self.processor.debit_credit(account_number, amount.clone(), TransactionType::DEBIT).await?;
        if message == "INSUFFICIENT_BALANCE" {
            Err(AppError::from_type(ErrorType::InsufficientBalance))
        } else {
            Ok(message)
        }
    }



    pub async fn find_account(&self, account_number: String) -> Result<Account, AppError> {
        let query_str = format!("SELECT * FROM bank_accounts b WHERE b.account_number = '{}'", account_number);
        let results = self.processor.query(&query_str).await?;
        if results.len() == 0 {
            Err(AppError::from_type(ErrorType::AccountNotFound(account_number)))
        } else {
            let result = &results[0];
            let account: Account = result.try_into().unwrap();
            Ok(account)
        }
    }

    pub async fn find_accounts(&self) -> Result<Vec<Account>, AppError> {
        let results = self.processor.query("SELECT * FROM bank_accounts").await?;
        let accounts = Account::from_ions(results);
        Ok(accounts)
    }

    pub async fn delete_account(&self, account_number: String) -> Result<String, AppError> {
        let query_str = format!("DELETE FROM bank_accounts b WHERE b.account_number = '{}'", account_number);
        match self.processor.delete(&query_str).await {
            Ok(doc_ids) => Ok(doc_ids[0].clone()),
            Err(AppError{message: None, error_type: ErrorType::NoRowsAffected}) => {
                let msg = format!("Unable to delete account: {}", account_number);
                Err(AppError::from_type(ErrorType::AccountError(msg)))
            },
            Err(e) => Err(e)
        }
    }


}