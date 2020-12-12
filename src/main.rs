#[macro_use]
extern crate log;

mod core;
mod domain;
mod error;
mod handler;
mod service;
mod util;

use domain::{AppState, NewAccount, NewTransaction};
use error::AppError;
use util::Config;

use actix_web::{web, App, FromRequest, HttpServer};
use dotenv::dotenv;
use std::process;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::builder().format_timestamp_millis().init();

    let config = Config::from_env().unwrap_or_else(|err| {
        error!("Config Error: {}", err);
        process::exit(1);
    });

    let app_state = AppState::new(config.clone()).await.unwrap();
    let server_port = config.server_port;

    let server = HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::new(
                r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %D"#,
            ))
            .data(app_state.clone())
            .wrap(actix_web::middleware::Compress::default())
            .service(
                web::scope("/")
                    .service(
                        web::scope("/account")
                            .service(
                                web::resource("")
                                    .app_data(web::Json::<NewAccount>::configure(|cfg| {
                                        cfg.error_handler(|err, _req| AppError::from(err).into())
                                    }))
                                    .route(web::get().to(handler::account::get_accounts))
                                    .route(web::post().to(handler::account::create_account)),
                            )
                            .service(
                                web::resource("/{account_number}")
                                    .route(web::get().to(handler::account::get_account))
                                    .route(web::delete().to(handler::account::delete_account)),
                            ),
                    )
                    .service(
                        web::scope("/transaction").service(
                            web::resource("")
                                .app_data(web::Json::<NewTransaction>::configure(|cfg| {
                                    cfg.error_handler(|err, _req| AppError::from(err).into())
                                }))
                                .route(web::post().to(handler::transaction::handle_transaction)),
                        ),
                    ),
            )
    })
    .bind(format!("0.0.0.0:{}", server_port))
    .unwrap()
    .run();

    info!(
        "Server running at {}",
        format!("http://localhost:{}", server_port)
    );

    server.await
}
