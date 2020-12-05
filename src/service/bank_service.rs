use crate::{core::QldbProcessor, Account, Config};
use bigdecimal::BigDecimal;
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
        info!("Successfully debited ${} from {}", amount, account_id);
        Ok(())
    }

    pub async fn credit(&self, account_id: String, amount: BigDecimal) -> Result<(), Error> {
        info!("Successfully credited ${} to {}", amount, account_id);
        Ok(())
    }

    pub async fn find_accounts(&self) -> Result<Vec<Account>, Error> {
        let results = self.processor.query("SELECT * from bank_accounts").await?;
        let accounts = Account::from_ions(results);
        Ok(accounts)
    }
}