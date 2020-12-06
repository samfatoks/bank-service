#[macro_use]
extern crate log;

mod core;
mod domain;
mod error;
mod service;
mod util;


use error::Error;
use bigdecimal::BigDecimal;
use domain::Account;
use service::AccountService;
use util::Config;
use std::str::FromStr;


use std::{env, process};
// use qldb::QLDBClient;
// use std::collections::HashMap;
// use ion_binary_rs::{IonEncoder, IonParser, IonValue};

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "info");
    env_logger::builder().format_timestamp_millis().init();

    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
    info!("{} v{}", NAME.unwrap_or("Crawler"), VERSION.unwrap_or("0.1.0"));

    let config: Config = Config::load().unwrap_or_else(|err| {
        error!("Config Error: {}", err);
        process::exit(1);
    });


    if let Err(err) =  run(config).await {
        error!("{}", err);
    }
}
pub async fn run(config: Config) -> Result<(), Error> {
    let bank_service = AccountService::new(config).await?;
    // for i in 1..=3 {
    //     let doc_id = bank_service.create_account("Omokunmi Fatoki".to_string(), "07062075792".to_string()).await?;
    //     info!("Account: {} -> {}", i, doc_id);
    // }

    // let accounts = bank_service.find_accounts().await?;
    // for account in accounts {
    //     info!("{}", account);
    //     // let doc_id = bank_service.delete_account(account.account_number.clone()).await?;
    //     // info!("Successfully deleted account: {}, docID: {}", account.account_number, doc_id);
    // }

    

    let account = bank_service.find_account("6436489976".to_string()).await?;
    info!("{}", account);
    let amount: BigDecimal = BigDecimal::from_str("500.0").unwrap();
    let result = bank_service.credit(account.account_number, amount).await?;
    info!("ID: {}", result);
    let account = bank_service.find_account("6436489976".to_string()).await?;
    info!("{}", account);
    // let doc_id = bank_service.delete_account(account.account_number).await?;
    // info!("Successfully deleted account: {}, docID: {}", "1601783131", doc_id);
    // let account_number = "12345678".to_string();
    // let amount: BigDecimal = BigDecimal::from_str("35.8").unwrap();
    // bank_service.debit(account_number, amount).await?;


    // let mut account = Account::new("Samuel Fatoki".to_string(), "07039645560".to_string());
    // account = account.add("12.356");
    // // info!("{}", account);
    // qldb_processor.insert(&account).await?;
    // qldb_processor.insert(&account.add("12.356")).await?;
    Ok(())
}