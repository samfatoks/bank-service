use crate::domain::{QldbInsertable, TransactionType};
use crate::error::{AppError, ErrorType};
use bigdecimal::BigDecimal;
use ion_binary_rs::IonValue;
use qldb::{DocumentCollection, QldbClient};
use std::collections::HashMap;

#[derive(Clone)]
pub struct QldbProcessor {
    client: QldbClient,
}

impl QldbProcessor {
    pub async fn new(ledger_name: String, session_pool_size: u16) -> Result<Self, AppError> {
        let client = QldbClient::default(&ledger_name, session_pool_size).await?;
        Ok(QldbProcessor { client })
    }

    pub async fn insert<I: QldbInsertable>(&self, model: &I) -> Result<String, AppError> {
        let mut value_to_insert = HashMap::new();
        value_to_insert.insert("test_column", "test_value");

        let results = self
            .client
            .transaction_within(|client| async move {
                let results = client
                    .query(format!("INSERT INTO {} VALUE ?", model.table_name()).as_ref())
                    .param(model.to_params())
                    .execute()
                    .await?;
                Ok(results)
            })
            .await?;

        let result = &results[0];
        let document_id: String = result.get_value("documentId")?;
        Ok(document_id)
    }

    pub async fn query(&self, query_str: &str) -> Result<DocumentCollection, AppError> {
        let builder = self.client.read_query(query_str).await?;
        let results = builder.execute().await?;
        Ok(results)
    }

    pub async fn delete(&self, query_str: &str) -> Result<Vec<String>, AppError> {
        let results = self
            .client
            .transaction_within(|client| async move {
                let results = client.query(query_str).execute().await?;
                Ok(results)
            })
            .await?;

        let docs = results.into_inner();
        if docs.len() == 0 {
            Err(AppError::from_type(ErrorType::NoRowsAffected))
        } else {
            let mut doc_ids = Vec::new();
            for doc in docs {
                let document_id: String = doc.get_value("documentId")?;
                doc_ids.push(document_id);
            }
            Ok(doc_ids)
        }
    }

    pub async fn debit_credit(
        &self,
        account_number: String,
        amount: BigDecimal,
        transaction_type: TransactionType,
    ) -> Result<String, AppError> {
        let results = self
            .client
            .transaction_within(|client| async move {
                let select_results = client
                    .query("SELECT balance FROM accounts b WHERE b.account_number = ?")
                    .param(IonValue::String(account_number.clone()))
                    .execute()
                    .await?;

                let select_doc = select_results[0].clone();
                let balance = select_doc.get_value("balance")?;
                let new_bal = match transaction_type {
                    TransactionType::CREDIT => balance + amount.clone(),
                    TransactionType::DEBIT => balance - amount.clone(),
                    _ => balance,
                };
                let zero: BigDecimal = 0u32.into();
                if new_bal < zero {
                    return Ok("INSUFFICIENT_BALANCE".to_string());
                }

                client
                    .query("UPDATE accounts SET balance = ? WHERE account_number = ?")
                    .param(IonValue::Decimal(new_bal))
                    .param(IonValue::String(account_number.clone()))
                    .execute()
                    .await?;

                let msg_bits = match transaction_type {
                    TransactionType::CREDIT => ("credited", "to"),
                    TransactionType::DEBIT => ("debited", "from"),
                    _ => ("transferred", "between"),
                };
                let message = format!(
                    "Successfully {} ${} {} {}",
                    msg_bits.0, amount, msg_bits.1, account_number
                );
                Ok(message)
            })
            .await?;
        Ok(results)
    }

    pub async fn transfer(
        &self,
        sender_account_number: String,
        recipient_account_number: String,
        amount: BigDecimal,
    ) -> Result<String, AppError> {
        let results = self
            .client
            .transaction_within(|client| async move {
                let src_balance_results = client
                    .query("SELECT balance FROM accounts b WHERE b.account_number = ?")
                    .param(IonValue::String(sender_account_number.clone()))
                    .execute()
                    .await?;

                let src_balance_doc = src_balance_results[0].clone();
                let src_balance: BigDecimal = src_balance_doc.get_value("balance")?;
                let new_src_bal = src_balance - amount.clone();
                let zero: BigDecimal = 0u32.into();
                if new_src_bal < zero {
                    return Ok("INSUFFICIENT_BALANCE".to_string());
                }

                let dst_balance_results = client
                    .query("SELECT balance FROM accounts b WHERE b.account_number = ?")
                    .param(IonValue::String(recipient_account_number.clone()))
                    .execute()
                    .await?;

                let dst_balance_doc = dst_balance_results[0].clone();
                let dst_balance: BigDecimal = dst_balance_doc.get_value("balance")?;
                let new_dst_bal = dst_balance + amount.clone();

                let qb = client
                    .query("UPDATE accounts SET balance = ? WHERE account_number = ?")
                    .param(IonValue::Decimal(new_src_bal))
                    .param(IonValue::String(sender_account_number.clone()));
                debug!("{:?}", qb);
                qb.execute().await?;

                let qb = client
                    .query("UPDATE accounts SET balance = ? WHERE account_number = ?")
                    .param(IonValue::Decimal(new_dst_bal))
                    .param(IonValue::String(recipient_account_number.clone()));
                debug!("{:?}", qb);
                qb.execute().await?;

                let message = format!(
                    "Successfully transferred ${} from {} to {}",
                    amount, sender_account_number, recipient_account_number
                );
                Ok(message)
            })
            .await?;
        Ok(results)
    }
}
