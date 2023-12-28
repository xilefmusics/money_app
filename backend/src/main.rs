mod attachment;
mod error;
mod history;
mod import;
mod rest;
mod settings;
mod transaction;

use error::AppError;
use fancy_surreal::Client;
use settings::Settings;

use actix_web::{web::Data, App, HttpServer};

use env_logger::Env;

#[actix_web::main]
async fn main() -> Result<(), AppError> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let settings = Settings::new();
    let database = Data::new(
        Client::new(
            &settings.db_host,
            settings.db_port,
            &settings.db_user,
            &settings.db_password,
            &settings.db_database,
            &settings.db_namespace,
        )
        .await
        .map_err(|err| AppError::Other(format!("Couldn't connect to database ({})", err)))?,
    );

    HttpServer::new(move || {
        let database = database.clone();
        App::new()
            .app_data(database)
            .service(transaction::rest::get_id)
            .service(transaction::rest::get)
            .service(transaction::rest::put)
            .service(transaction::rest::post)
            .service(transaction::rest::delete)
            .service(transaction::rest::get_pods)
            .service(transaction::rest::get_budgets)
            .service(transaction::rest::get_inbudgets)
            .service(transaction::rest::get_debts)
            .service(transaction::rest::get_tags)
            .service(history::rest::get_wealth)
            .service(history::rest::get_pods)
            .service(history::rest::get_budgets)
            .service(history::rest::get_inbudgets)
            .service(history::rest::get_debts)
            .service(import::rest::import)
            .service(attachment::rest::get_id)
            .service(attachment::rest::post)
            .service(rest::get_index)
            .service(rest::get_static_files)
    })
    .bind((settings.host, settings.port))
    .map_err(|err| AppError::Other(format!("Couldn't bind port ({})", err)))?
    .run()
    .await
    .map_err(|err| AppError::Other(format!("Server crashed ({})", err)))
}
