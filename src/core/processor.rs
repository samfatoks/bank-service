use bigdecimal::BigDecimal;
use qldb::QLDBClient;
use std::collections::HashMap;
use ion_binary_rs::IonValue;
use crate::domain::{QldbInsertable, TransactionType};
use std::convert::TryInto;
use crate::error::{AppError, ErrorType};

pub struct QldbProcessor {
    client: QLDBClient,
}

impl QldbProcessor {
    pub async fn new(ledger_name: String) -> Result<Self, AppError> {
        let client = QLDBClient::default(&ledger_name).await?;
        Ok(QldbProcessor {
            client
        })
    }

    pub async fn insert<I: QldbInsertable>(&self, model: &I) -> Result<String, AppError> {
        let results = self.client
        .transaction_within(|client| async move {   
            let results = client
                .query(format!("INSERT INTO {} VALUE ?", model.table_name()).as_ref())
                .param(model.to_params())
                .execute()
                .await?;
            Ok(results)
        }).await?;

        let result = &results[0];
        let map: HashMap<String, IonValue> = result.try_into().unwrap();
        let document_id: String = map.get("documentId").unwrap().try_into()?;
        Ok(document_id)
    }

    pub async fn query(&self, query_str: &str) -> Result<Vec<IonValue>, AppError> {
        let mut builder = self.client.read_query(query_str).await?;
        let results = builder.execute().await?;
        Ok(results)
    }

    pub async fn delete(&self, query_str: &str) -> Result<Vec<String>, AppError> {
        let results = self.client
        .transaction_within(|client| async move {   
            let results = client
                .query(query_str)
                .execute()
                .await?;
            Ok(results)
        }).await?;

        if results.len() == 0 {
            Err(AppError::from_type(ErrorType::NoRowsAffected))
        } else {
            let mut doc_ids = Vec::new();
            for result in &results {
                let map: HashMap<String, IonValue> = result.try_into().unwrap();
                let document_id: String = map.get("documentId").unwrap().try_into()?;
                doc_ids.push(document_id);
            }

            Ok(doc_ids)
        }
    }

    pub async fn debit_credit(&self, account_number: String, amount: BigDecimal, transaction_type: TransactionType) -> Result<String, AppError> {
        let results= self.client
        .transaction_within(|client| async move {   
            let select_results = client
                .query("SELECT balance FROM bank_accounts b WHERE b.account_number = ?")
                .param(IonValue::String(account_number.clone()))
                .execute()
                .await?;

                let select_result = select_results[0].clone();
                let map: HashMap<String, IonValue> = select_result.try_into().unwrap();
                
                let mut balance = BigDecimal::default();
                if let IonValue::Decimal(bal) = map.get("balance").unwrap() {
                    balance = bal.clone();
                }
                let new_bal = match transaction_type {
                    TransactionType::CREDIT => balance + amount.clone(),
                    TransactionType::DEBIT => balance - amount.clone(),
                    _ => balance
                };
                let zero: BigDecimal = 0u32.into();
                if new_bal < zero {
                    Ok("INSUFFICIENT_BALANCE".to_string())
                } else {
                    client
                        .query("UPDATE bank_accounts SET balance = ? WHERE account_number = ?")
                        .param(IonValue::Decimal(new_bal))
                        .param(IonValue::String(account_number.clone()))
                        .execute()
                        .await?;
                    
                      let msg_bits = match transaction_type {
                          TransactionType::CREDIT => ("credited", "to"),
                          TransactionType::DEBIT => ("debited", "from"),
                          _ => ("transferred", "between")
                      };
                      let message = format!("Successfully {} ${} {} {}", msg_bits.0, amount, msg_bits.1, account_number);
                      Ok(message)
                }
        }).await?;
        Ok(results)
    }

    pub async fn transfer(&self, sender_account_number: String, recipient_account_number: String, amount: BigDecimal) -> Result<String, AppError> {
        let results= self.client
        .transaction_within(|client| async move {   
            let src_balance_results = client
                .query("SELECT balance FROM bank_accounts b WHERE b.account_number = ?")
                .param(IonValue::String(sender_account_number.clone()))
                .execute()
                .await?;
 
            let src_balance_result = src_balance_results[0].clone();
            let map: HashMap<String, IonValue> = src_balance_result.try_into().unwrap();
            let mut src_balance = BigDecimal::default();
            if let IonValue::Decimal(bal) = map.get("balance").unwrap() {
                src_balance = bal.clone();
            }
            let new_src_bal = src_balance - amount.clone();
            let zero: BigDecimal = 0u32.into();
            if new_src_bal < zero {
                Ok("INSUFFICIENT_BALANCE".to_string())
            } else {
                let dst_balance_results = client
                    .query("SELECT balance FROM bank_accounts b WHERE b.account_number = ?")
                    .param(IonValue::String(recipient_account_number.clone()))
                    .execute()
                    .await?;
    
                let dst_balance_result = dst_balance_results[0].clone();
                let map: HashMap<String, IonValue> = dst_balance_result.try_into().unwrap();
                let mut dst_balance = BigDecimal::default();
                if let IonValue::Decimal(bal) = map.get("balance").unwrap() {
                    dst_balance = bal.clone();
                }
                let new_dst_bal = dst_balance + amount.clone();

                client
                    .query("UPDATE bank_accounts SET balance = ? WHERE account_number = ?")
                    .param(IonValue::Decimal(new_src_bal))
                    .param(IonValue::String(sender_account_number.clone()))
                    .execute()
                    .await?;

                client
                    .query("UPDATE bank_accounts SET balance = ? WHERE account_number = ?")
                    .param(IonValue::Decimal(new_dst_bal))
                    .param(IonValue::String(recipient_account_number.clone()))
                    .execute()
                    .await?;

                let message = format!("Successfully trasferred ${} from {} to {}", amount, sender_account_number, recipient_account_number);
                Ok(message)
            }
        }).await?;
        Ok(results)
    }

}