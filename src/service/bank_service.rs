use crate::{core::QldbProcessor, Account, Config};
use bigdecimal::BigDecimal;
use std::convert::TryInto;
use crate::error::Error;

pub struct BankService {
    processor: QldbProcessor
}

impl BankService {
    pub async fn new(config: Config) -> Result<BankService, Error>  {
        let processor = QldbProcessor::new(config).await?;
        Ok(BankService { processor })
    }

    pub async fn create_account(&self, name: String, phone: String) -> Result<String, Error>{
        let account = Account::new(name, phone);
        let document_id = self.processor.insert(&account).await?;
        Ok(document_id)
    }

    pub async fn transfer(&self, src_account_id: String, dst_account_id: String, amount: BigDecimal) -> Result<(), Error> {
        info!("Successfully trasferred ${} from {} to {}", amount, src_account_id, dst_account_id);
        Ok(())
    }

    pub async fn debit(&self, account_id: String, amount: BigDecimal) -> Result<(), Error> {
        // info!("Successfully debited ${} from {}", amount, account_id);
        // Ok(())
        Err(Error::InsufficientFunds(amount))
    }

    pub async fn credit(&self, account_id: String, amount: BigDecimal) -> Result<(), Error> {
        info!("Successfully credited ${} to {}", amount, account_id);
        Ok(())
    }

    pub async fn find_account(&self, account_id: String) -> Result<Account, Error> {
        let query_str = format!("SELECT * FROM bank_accounts b WHERE b.account_id = '{}'", account_id);
        let results = self.processor.query(&query_str).await?;
        if results.len() == 0 {
            Err(Error::AccountNotFound(account_id))
        } else {
            let result = &results[0];
            let account: Account = result.try_into().unwrap();
            Ok(account)
        }
    }

    pub async fn delete_account(&self, account_id: String) -> Result<String, Error> {
        let query_str = format!("DELETE FROM bank_accounts b WHERE b.account_id = '{}'", account_id);
        match self.processor.delete(&query_str).await {
            Ok(doc_ids) => Ok(doc_ids[0].clone()),
            Err(Error::NoRowsAffected) => Err(Error::Custom(format!("Unable to delete account: {}", account_id))),
            Err(e) => Err(e)
        }
    }

    pub async fn find_accounts(&self) -> Result<Vec<Account>, Error> {
        let results = self.processor.query("SELECT * FROM bank_accounts").await?;
        let accounts = Account::from_ions(results);
        Ok(accounts)
    }
}