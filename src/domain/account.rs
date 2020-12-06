use bigdecimal::BigDecimal;
use std::{convert::TryFrom, convert::TryInto};
use std::fmt::{Display, Formatter, Error as FmtError};
use std::collections::HashMap;
use ion_binary_rs::IonValue;
use chrono::prelude::*;


use super::default_datetime;
use super::QldbInsertable;
use crate::error::AppError;
use crate::util::rand_util;
use serde::{Deserialize, Serialize};

const TABLE_NAME: &str = "bank_accounts";
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
        let account_number = rand_util::generate_numeric(10).unwrap();
        let now: DateTime<FixedOffset> = Utc::now().into();
        Account {
            account_number,
            name,
            phone,
            balance: BigDecimal::default().with_scale(2),
            created_at: now,
            updated_at: now
        }
    }

    pub fn from_ions(result: Vec<IonValue>) -> Vec<Self> {
        result.iter().map(|i| i.try_into()).filter_map(Result::ok).collect()
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
        params.insert("account_number".to_string(), IonValue::String(self.account_number.to_string()));
        params.insert("name".to_string(), IonValue::String(self.name.to_string()));
        params.insert("phone".to_string(), IonValue::String(self.phone.to_string()));
        params.insert("balance".to_string(), IonValue::Decimal(self.balance.clone()));
        params.insert("created_at".to_string(), IonValue::DateTime(self.created_at.clone()));
        params.insert("updated_at".to_string(), IonValue::DateTime(self.updated_at.clone()));
        params
    }
}

impl TryFrom<&IonValue> for Account {
    type Error = AppError;

    fn try_from(value: &IonValue) -> Result<Self, Self::Error> {
        let map: HashMap<String, IonValue> = value.try_into().unwrap();

        let account_number: String = map.get("account_number").unwrap().try_into()?;
        let name: String = map.get("name").unwrap().try_into()?;
        let phone: String = map.get("phone").unwrap().try_into()?;
        let mut balance = BigDecimal::default();
        if let IonValue::Decimal(bal) = map.get("balance").unwrap() {
            balance = bal.clone();
        }
        let created_at: DateTime<FixedOffset> = map.get("created_at").unwrap().try_into()?;
        let updated_at: DateTime<FixedOffset> = map.get("updated_at").unwrap().try_into()?;
        let account = Account {
                account_number: account_number,
                name: name,
                phone: phone,
                balance: balance.with_scale(2),
                created_at: created_at,
                updated_at: updated_at
            };
        Ok(account)
    }
}

#[derive(Debug, Deserialize)]
pub struct NewAccount {
    pub name: String,
    pub phone: String
}

impl Into<Account> for NewAccount {
    fn into(self) -> Account {
        Account::new(self.name, self.phone)
    }
}