mod account;
mod transaction;
mod app_state;
mod response;

pub use transaction::{TransactionType, NewTransaction};
pub use account::{Account, NewAccount};
pub use app_state::AppState;
pub use response::Response;

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