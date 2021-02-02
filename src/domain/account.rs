use bigdecimal::BigDecimal;
use chrono::prelude::*;
use ion_binary_rs::IonValue;
use qldb::Document;
use std::collections::HashMap;
use std::fmt::{Display, Error as FmtError, Formatter};
use std::{convert::TryFrom, convert::TryInto};

use super::default_datetime;
use super::QldbInsertable;
use crate::error::AppError;
use crate::util;
use serde::{Deserialize, Serialize};

const TABLE_NAME: &str = "accounts";
#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub account_number: String,
    pub name: String,
    pub phone: String,
    pub balance: BigDecimal,
    #[serde(skip, default = "default_datetime")]
    pub created_at: DateTime<FixedOffset>,
    #[serde(skip, default = "default_datetime")]
    pub updated_at: DateTime<FixedOffset>,
}

impl Account {
    pub fn new(name: String, phone: String) -> Account {
        let account_number = util::generate_numeric(10).unwrap();
        let now: DateTime<FixedOffset> = Utc::now().into();
        Account {
            account_number,
            name,
            phone,
            balance: BigDecimal::default().with_scale(2),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn from_documents(result: Vec<Document>) -> Vec<Self> {
        result
            .iter()
            .map(|i| i.try_into())
            .filter_map(Result::ok)
            .collect()
    }
}

impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), FmtError> {
        write!(f, "Account {{ account_number: {}, name: {}, phone: {}, balance: {}, created_at: {}, updated_at: {} }}", 
        self.account_number, self.name, self.phone, self.balance, self.created_at, self.updated_at)
    }
}

impl QldbInsertable for Account {
    fn table_name(&self) -> &str {
        TABLE_NAME
    }

    fn to_params(&self) -> HashMap<String, IonValue> {
        let mut params = HashMap::new();
        params.insert(
            "account_number".to_string(),
            IonValue::String(self.account_number.to_string()),
        );
        params.insert("name".to_string(), IonValue::String(self.name.to_string()));
        params.insert(
            "phone".to_string(),
            IonValue::String(self.phone.to_string()),
        );
        params.insert(
            "balance".to_string(),
            IonValue::Decimal(self.balance.clone()),
        );
        params.insert(
            "created_at".to_string(),
            IonValue::DateTime(self.created_at.clone()),
        );
        params.insert(
            "updated_at".to_string(),
            IonValue::DateTime(self.updated_at.clone()),
        );
        params
    }
}

impl TryFrom<&Document> for Account {
    type Error = AppError;

    fn try_from(doc: &Document) -> Result<Self, Self::Error> {
        let account_number: String = doc.get_value("account_number")?;
        let name: String = doc.get_value("name")?;
        let phone: String = doc.get_value("phone")?;
        let balance: BigDecimal = doc.get_value("balance")?;
        let created_at: DateTime<FixedOffset> = doc.get_value("created_at")?;
        let updated_at: DateTime<FixedOffset> = doc.get_value("updated_at")?;
        let account = Account {
            account_number,
            name,
            phone,
            balance: balance.with_scale(2),
            created_at,
            updated_at,
        };
        Ok(account)
    }
}

#[derive(Debug, Deserialize)]
pub struct NewAccount {
    pub name: String,
    pub phone: String,
}

impl Into<Account> for NewAccount {
    fn into(self) -> Account {
        Account::new(self.name, self.phone)
    }
}
