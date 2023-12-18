mod database;
mod error;
mod settings;
mod transaction;

use database::Database;
use settings::Settings;

use actix_web::{web::Data, App, HttpServer};

use env_logger::Env;

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let settings = Settings::new();
    let database = Data::new(Database::new(settings.clone()).await);

    HttpServer::new(move || {
        let database = database.clone();
        App::new()
            .app_data(database)
            .service(transaction::rest::get)
            .service(transaction::rest::post)
    })
    .bind((settings.host, settings.port))
    .unwrap()
    .run()
    .await
    .unwrap()
}
