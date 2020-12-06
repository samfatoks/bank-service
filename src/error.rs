use std::error::{Error as StdError};
use std::fmt::{Display, Formatter, Error as FmtError};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use std::{fmt, io};
use ion_binary_rs::IonParserError;
use qldb::QLDBError;

#[derive(Debug)]
pub enum Error {
    InvalidCommand,
    InsufficientBalance(BigDecimal),
    Unsupported(&'static str),
    IO(io::Error),
    SerdeError(serde_json::error::Error),
    Custom(String),
    IonError(IonParserError),
    QLDBError(QLDBError),
    AccountNotFound(String),
    NoRowsAffected
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::IO(err) => write!(f, "IO Error: {}", err),
            Error::Custom(message) => write!(f, "Error: {}", message),
            Error::SerdeError(ref err) => write!(f, "Error: {}", err),
            Error::InvalidCommand => write!(f, "Invalid request command"),
            Error::InsufficientBalance(amount) => write!(f, "Insufficient balance in account"),
            Error::Unsupported(s) => write!(f, "Not supported"),
            Error::IonError(s) => write!(f, "Ion Parser Error: {}", s),
            Error::QLDBError(s) => write!(f, "QLDB Error: {}", s),
            Error::AccountNotFound(s) => write!(f, "Account not found: {}", s),
            Error::NoRowsAffected => write!(f, "No rows affected")
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(err: serde_json::error::Error) -> Self {
        Error::SerdeError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::IO(err)
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(err: std::str::Utf8Error) -> Self {
        Error::Custom(err.to_string())
    }
}

impl From<IonParserError> for Error {
    fn from(err: IonParserError) -> Self {
        Error::IonError(err)
    } 
}

impl From<QLDBError> for Error {
    fn from(err: QLDBError) -> Self {
        Error::QLDBError(err)
    }
}