mod database;
mod error;
mod settings;

use database::Database;
use settings::Settings;
use error::AppError;

use actix_web::{web::Data, App, HttpServer, HttpResponse, get};

use env_logger::Env;

#[get("/")]
pub async fn index() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json("Hello World"))
}

#[actix_web::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    let settings = Settings::new();
    let database = Data::new(Database::new(settings.clone()).await);

    HttpServer::new(move || {
        let database = database.clone();
        App::new()
            .app_data(database)
            .service(index)
    })
    .bind((settings.host, settings.port))
    .unwrap()
    .run()
    .await
    .unwrap()
}
