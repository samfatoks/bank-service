use std::str;
use std::fmt::{Formatter, Display};
use crate::error::Error;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub enum Command {
    FundsTransfer,
    Balance,
    BillPayment,
    Query,
    Register
}

impl From<Command> for String {
    fn from(c: Command) -> Self {
        c.code().to_string()
    }
}

impl str::FromStr for Command {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "FT" => Ok(Command::FundsTransfer),
            "BAL" => Ok(Command::Balance),
            "BILL" => Ok(Command::BillPayment),
            "QUERY" => Ok(Command::Query),
            _ => Err(Error::InvalidCommand)
        }
    }
}

impl Default for Command {
    fn default() -> Self { Command::Query }
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        //f.write_str("ddf")
        write!(f, "{}", self.description())
    }
}

impl Command {
    pub fn description(&self) -> &str {
        match self {
            Command::FundsTransfer => "FT: Transfer funds from Wallet to Wallet or Wallet to Account",
            Command::Balance => "BAL: Check Balance",
            _ => "OT: Others"
        }
    }
    pub fn code(&self) -> &str {
        match self {
            Command::FundsTransfer => "FT",
            Command::Balance => "BAL",
            _ => "OT"
        }
    }

}