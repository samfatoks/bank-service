mod account;
mod app_state;
mod response;
mod transaction;

pub use account::{Account, NewAccount};
pub use app_state::AppState;
pub use response::Response;
pub use transaction::{NewTransaction, TransactionType};

use chrono::{DateTime, FixedOffset, Utc};
use ion_binary_rs::IonValue;
use std::collections::HashMap;

pub trait QldbInsertable {
    fn table_name(&self) -> &str;
    fn to_params(&self) -> HashMap<String, IonValue>;
}

pub fn default_datetime() -> DateTime<FixedOffset> {
    Utc::now().into()
}
