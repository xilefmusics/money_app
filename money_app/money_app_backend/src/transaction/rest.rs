use super::{model, Transaction};

use crate::database::Database;
use crate::error::AppError;

use actix_web::{get, post, web::Data, web::Json, HttpRequest, HttpResponse};

#[get("/api/transaction")]
pub async fn get(req: HttpRequest, db: Data<Database>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(model::select(db.select().user("xilef")).await?))
}

#[post("/api/transaction")]
pub async fn post(
    req: HttpRequest,
    transactions: Json<Vec<Transaction>>,
    db: Data<Database>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok()
        .json(model::create(&db, "xilef".into(), transactions.into_inner()).await?))
}
