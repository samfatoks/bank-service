use bigdecimal::BigDecimal;
use super::{Account, Status};

#[derive(Debug)]
pub struct Transaction {
  pub reference: String,
  pub amount: BigDecimal,
  pub sender_account_id: String,
  pub recipient_account_id: String,
  pub status: Status,
}

// impl Transaction {
//   pub fn new() -> Transaction {
    
//   }
// }