use crate::core::QldbProcessor;
use crate::domain::{Account, NewAccount};
use crate::error::{AppError, ErrorType};
use std::convert::TryInto;

pub struct AccountService {
    processor: QldbProcessor,
}

impl AccountService {
    pub fn new(processor: QldbProcessor) -> AccountService {
        AccountService { processor }
    }

    pub async fn create_account(
        &self,
        new_account: NewAccount,
    ) -> Result<(String, Account), AppError> {
        let account: Account = new_account.into();
        let document_id = self.processor.insert(&account).await?;
        Ok((document_id, account))
    }

    pub async fn find_account(&self, account_number: String) -> Result<Account, AppError> {
        let query_str = format!(
            "SELECT * FROM accounts b WHERE b.account_number = '{}'",
            account_number
        );
        let results = self.processor.query(&query_str).await?;
        let docs = results.into_inner();
        if docs.len() == 0 {
            Err(AppError::from_type(ErrorType::AccountNotFound(
                account_number,
            )))
        } else {
            let result = docs.get(0).unwrap();
            let account: Account = result.try_into().unwrap();
            Ok(account)
        }
    }

    pub async fn find_accounts(&self) -> Result<Vec<Account>, AppError> {
        let results = self.processor.query("SELECT * FROM accounts").await?;
        // let points: u64 = document.get_value("points")?;
        // let result: u64 = results
        //     .into_iter()
        //     .map(|doc| doc.get_value::<u64>("points"))
        //     .collect::<Result<Vec<u64>, _>>()?
        //     .into_iter()
        //     .fold(0, |acc, val| acc + val);
        let accounts = Account::from_documents(results.into_inner());
        Ok(accounts)
    }

    pub async fn delete_account(&self, account_number: String) -> Result<String, AppError> {
        let query_str = format!(
            "DELETE FROM accounts b WHERE b.account_number = '{}'",
            account_number
        );
        match self.processor.delete(&query_str).await {
            Ok(doc_ids) => Ok(doc_ids[0].clone()),
            Err(AppError {
                message: None,
                error_type: ErrorType::NoRowsAffected,
            }) => {
                let msg = format!("Unable to delete account: {}", account_number);
                Err(AppError::from_type(ErrorType::AccountError(msg)))
            }
            Err(e) => Err(e),
        }
    }
}
