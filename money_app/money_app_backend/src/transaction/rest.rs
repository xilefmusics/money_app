use super::Transaction;

use crate::error::AppError;
use crate::rest::parse_user_header;

use fancy_surreal::Client;

use actix_web::{
    delete, get, post, put, web::Data, web::Json, web::Path, HttpRequest, HttpResponse,
};

#[get("/api/transactions")]
pub async fn get(req: HttpRequest, db: Data<Client>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        db.table("transactions")
            .owner(&parse_user_header(req)?)
            .select()?
            .query::<Transaction>()
            .await?,
    ))
}

#[get("/api/transactions/{id:transaction.*}")]
pub async fn get_id(
    req: HttpRequest,
    db: Data<Client>,
    id: Path<String>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        db.table("transactions")
            .owner(&parse_user_header(req)?)
            .select()?
            .id(&id.into_inner())
            .query_one::<Transaction>()
            .await?,
    ))
}

#[put("/api/transactions")]
pub async fn put(
    req: HttpRequest,
    transactions: Json<Vec<Transaction>>,
    db: Data<Client>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Created().json(
        db.table("transactions")
            .owner(&parse_user_header(req)?)
            .update(transactions.into_inner())
            .await?,
    ))
}

#[delete("/api/transactions")]
pub async fn delete(
    req: HttpRequest,
    transactions: Json<Vec<Transaction>>,
    db: Data<Client>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::NoContent().json(
        db.table("transactions")
            .owner(&parse_user_header(req)?)
            .delete(transactions.into_inner())
            .await?,
    ))
}

#[post("/api/transactions")]
pub async fn post(
    req: HttpRequest,
    transactions: Json<Vec<Transaction>>,
    db: Data<Client>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Created().json(
        db.table("transactions")
            .owner(&parse_user_header(req)?)
            .create(transactions.into_inner())
            .await?,
    ))
}
