#[macro_use]
extern crate log;

mod core;
mod domain;
mod error;
mod service;
mod util;
mod handler;


use domain::{NewAccount, NewTransaction};
use util::Config;
use error::AppError;

use actix_web::{web, App, FromRequest, HttpServer};
use std::{env, process};

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::builder().format_timestamp_millis().init();

    const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
    const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");
    info!("{} v{}", NAME.unwrap_or("Bank"), VERSION.unwrap_or("0.1.0"));

    let config: Config = Config::load().unwrap_or_else(|err| {
        error!("Config Error: {}", err);
        process::exit(1);
    });
    let http_port = config.http_port;

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::new(
                r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %D"#,
            ))
            .data(config.clone())
            .wrap(actix_web::middleware::Compress::default())
            .service(web::scope("/")
                .service(
                    web::scope("/account")
                        .service(
                            web::resource("")
                                .app_data(web::Json::<NewAccount>::configure(|cfg| {
                                    cfg.error_handler(|err, _req| AppError::from(err).into())
                                }))
                                .route(web::get().to(handler::account::get_accounts))
                                .route(web::post().to(handler::account::create_account))
                        )
                        .service(
                            web::resource("/{account_number}")
                                .route(web::get().to(handler::account::get_account))
                                .route(web::delete().to(handler::account::delete_account)),
                        ),
                        
                )
                .service(
                web::scope("/transaction")
                        .service(
                            web::resource("")
                                .app_data(web::Json::<NewTransaction>::configure(|cfg| {
                                    cfg.error_handler(|err, _req| AppError::from(err).into())
                                }))
                                .route(web::post().to(handler::transaction::handle_transaction))
                        )
                ),
            )
    })
    .bind(format!("0.0.0.0:{}", http_port))
    .unwrap()
    .run()
    .await
}