mod account;
mod status;
mod transaction;

pub use status::Status;
pub use transaction::{Transaction, TransactionType, NewTransaction};
pub use account::{Account, NewAccount,};

use std::collections::HashMap;
use ion_binary_rs::IonValue;
use chrono::{DateTime, FixedOffset, Utc};

pub trait QldbInsertable {
    fn table_name(&self) -> &str;
    fn to_params(&self) -> HashMap<String, IonValue>;
}

pub fn default_datetime() -> DateTime<FixedOffset> {
    Utc::now().into()
}