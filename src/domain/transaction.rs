use bigdecimal::BigDecimal;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub enum TransactionType {
  CREDIT,
  DEBIT,
  TRANSFER
}

#[derive(Debug, Deserialize)]
pub struct NewTransaction {
  pub amount: BigDecimal,
  pub sender_account_number: Option<String>,
  pub recipient_account_number: String,
  pub transaction_type: TransactionType,
}