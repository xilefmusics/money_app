mod database;
mod error;
mod settings;
mod transaction;

use database::{model, Database};
use error::AppError;
use settings::Settings;
use transaction::Transaction;

use actix_web::{get, post, web::Data, web::Json, App, HttpRequest, HttpResponse, HttpServer};

use env_logger::Env;

#[get("/")]
pub async fn index() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json("Hello World"))
}

#[post("/api/transaction")]
pub async fn post_transaction(
    req: HttpRequest,
    transactions: Json<Vec<Transaction>>,
    db: Data<Database>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok()
        .json(model::add_transactions(&db, "xilef".into(), dbg!(transactions.into_inner())).await?))
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
            .service(post_transaction)
    })
    .bind((settings.host, settings.port))
    .unwrap()
    .run()
    .await
    .unwrap()
}
