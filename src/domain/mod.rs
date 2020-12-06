mod command;
mod account;
mod status;
mod transaction;

pub use status::Status;
pub use transaction::{Transaction, TransactionType};
pub use command::Command;
pub use account::Account;

use std::collections::HashMap;
use ion_binary_rs::IonValue;

pub trait QldbInsertable {
    fn table_name(&self) -> &str;
    fn to_params(&self) -> HashMap<String, IonValue>;
}
