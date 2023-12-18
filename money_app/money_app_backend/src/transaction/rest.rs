use super::{model, Transaction};

use crate::database::Database;
use crate::error::AppError;
use crate::rest::parse_user_header;

use actix_web::{get, post, web::Data, web::Json, HttpRequest, HttpResponse};

#[get("/api/transaction")]
pub async fn get(req: HttpRequest, db: Data<Database>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(model::select(db.select().user(&parse_user_header(req)?)).await?))
}

#[post("/api/transaction")]
pub async fn post(
    req: HttpRequest,
    transactions: Json<Vec<Transaction>>,
    db: Data<Database>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok()
        .json(model::create(&db, parse_user_header(req)?, transactions.into_inner()).await?))
}
