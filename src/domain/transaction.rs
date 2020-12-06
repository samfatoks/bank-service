use bigdecimal::BigDecimal;
use super::Status;

#[derive(Debug)]
pub enum TransactionType {
  CREDIT,
  DEBIT
}

#[derive(Debug)]
pub struct Transaction {
  pub reference: String,
  pub amount: BigDecimal,
  pub sender_account_number: String,
  pub recipient_account_number: String,
  pub transaction_type: TransactionType,
  pub status: Status,
}