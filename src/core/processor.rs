use bigdecimal::BigDecimal;
use qldb::QLDBClient;
use std::collections::HashMap;
use ion_binary_rs::IonValue;
use crate::Config;
use crate::domain::{QldbInsertable, TransactionType};
use std::convert::TryInto;
use crate::error::Error;

pub struct QldbProcessor {
    client: QLDBClient,
}

impl QldbProcessor {
    pub async fn new(config: Config) -> Result<Self, Error> {
        let client = QLDBClient::default(&config.ledger_name).await?;
        Ok(QldbProcessor {
            client
        })
    }

    pub async fn insert<I: QldbInsertable>(&self, model: &I) -> Result<String, Error> {
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

    pub async fn query(&self, query_str: &str) -> Result<Vec<IonValue>, Error> {
        let mut builder = self.client.read_query(query_str).await?;
        let results = builder.execute().await?;
        Ok(results)
    }

    pub async fn delete(&self, query_str: &str) -> Result<Vec<String>, Error> {
        let results = self.client
        .transaction_within(|client| async move {   
            let results = client
                .query(query_str)
                .execute()
                .await?;
            Ok(results)
        }).await?;

        if results.len() == 0 {
            Err(Error::NoRowsAffected)
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

    pub async fn update_balance(&self, account_number: String, amount: BigDecimal, transaction_type: TransactionType) -> Result<String, Error> {
        let results= self.client
        .transaction_within(|client| async move {   
            let select_results = client
                .query("SELECT balance FROM bank_accounts b WHERE b.account_number = ?")
                .param(IonValue::String(account_number.clone()))
                .execute()
                .await?;
            let result = if select_results.len() == 0 {
                Err(Error::NoRowsAffected)
            } else {
                let select_result = select_results[0].clone();
                let map: HashMap<String, IonValue> = select_result.try_into().unwrap();
                
                if let IonValue::Decimal(bal) = map.get("balance").unwrap() {

                    let new_bal = match transaction_type {
                        TransactionType::CREDIT => bal + amount,
                        TransactionType::DEBIT => bal - amount
                    };
                    let zero: BigDecimal = 0u32.into();
                    if new_bal < zero {
                        Ok("0".to_string())
                    } else {
                        let update_results = client
                            .query("UPDATE bank_accounts SET balance = ? WHERE account_number = ?")
                            .param(IonValue::Decimal(new_bal))
                            .param(IonValue::String(account_number))
                            .execute()
                            .await?;
                        if update_results.len() == 0 {
                            Err(Error::NoRowsAffected)
                        } else {
                            let result = &update_results[0];
                            let map: HashMap<String, IonValue> = result.try_into().unwrap();
                            let document_id: String = map.get("documentId").unwrap().try_into().unwrap();
                            Ok(document_id)
                        }
                    }
                } else {
                    Err(Error::Custom("Unbale to parse balance".to_string()))
                }
            };
            Ok(result.unwrap())
        }).await?;
        Ok(results)
    }

    pub async fn transfer(&self, account_number: String, amount: BigDecimal, transaction_type: TransactionType) -> Result<String, Error> {
        let results= self.client
        .transaction_within(|client| async move {   
            let select_results = client
                .query("SELECT balance FROM bank_accounts b WHERE b.account_number = ?")
                .param(IonValue::String(account_number.clone()))
                .execute()
                .await?;
            let result = if select_results.len() == 0 {
                Err(Error::NoRowsAffected)
            } else {
                let select_result = select_results[0].clone();
                let map: HashMap<String, IonValue> = select_result.try_into().unwrap();
                
                if let IonValue::Decimal(bal) = map.get("balance").unwrap() {

                    let new_bal = match transaction_type {
                        TransactionType::CREDIT => bal + amount,
                        TransactionType::DEBIT => bal - amount
                    };
                    let zero: BigDecimal = 0u32.into();
                    if new_bal < zero {
                        Ok("0".to_string())
                    } else {
                        let update_results = client
                            .query("UPDATE bank_accounts SET balance = ? WHERE account_number = ?")
                            .param(IonValue::Decimal(new_bal))
                            .param(IonValue::String(account_number))
                            .execute()
                            .await?;
                        if update_results.len() == 0 {
                            Err(Error::NoRowsAffected)
                        } else {
                            let result = &update_results[0];
                            let map: HashMap<String, IonValue> = result.try_into().unwrap();
                            let document_id: String = map.get("documentId").unwrap().try_into().unwrap();
                            Ok(document_id)
                        }
                    }
                } else {
                    Err(Error::Custom("Unbale to parse balance".to_string()))
                }
            };
            Ok(result.unwrap())
        }).await?;
        Ok(results)
    }

}