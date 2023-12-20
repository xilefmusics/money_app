use super::Transaction;

use crate::error::AppError;
use crate::rest::parse_user_header;

use fancy_surreal::Client;

use actix_web::{
    delete, get, post, put, web::Data, web::Json, web::Path, HttpRequest, HttpResponse,
};

#[get("/api/transactions")]
pub async fn get(req: HttpRequest, db: Data<Client>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(Transaction::get(db.into_inner(), &parse_user_header(req)?).await?))
}

#[get("/api/transactions/{id:transaction.*}")]
pub async fn get_id(
    req: HttpRequest,
    db: Data<Client>,
    id: Path<String>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        Transaction::get_one(db.into_inner(), &parse_user_header(req)?, &id.into_inner()).await?,
    ))
}

#[put("/api/transactions")]
pub async fn put(
    req: HttpRequest,
    transactions: Json<Vec<Transaction>>,
    db: Data<Client>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Created().json(
        Transaction::put(
            db.into_inner(),
            &parse_user_header(req)?,
            transactions.into_inner(),
        )
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
        Transaction::delete(
            db.into_inner(),
            &parse_user_header(req)?,
            transactions.into_inner(),
        )
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
        Transaction::create(
            db.into_inner(),
            &parse_user_header(req)?,
            transactions.into_inner(),
        )
        .await?,
    ))
}

#[get("/api/pods")]
pub async fn get_pods(req: HttpRequest, db: Data<Client>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        Transaction::get_assiciated_type(db.into_inner(), &parse_user_header(req)?, "pods").await?,
    ))
}

#[get("/api/budgets")]
pub async fn get_budgets(req: HttpRequest, db: Data<Client>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        Transaction::get_assiciated_type(db.into_inner(), &parse_user_header(req)?, "budgets")
            .await?,
    ))
}

#[get("/api/inbudgets")]
pub async fn get_inbudgets(req: HttpRequest, db: Data<Client>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        Transaction::get_assiciated_type(db.into_inner(), &parse_user_header(req)?, "inbudgets")
            .await?,
    ))
}

#[get("/api/debts")]
pub async fn get_debts(req: HttpRequest, db: Data<Client>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        Transaction::get_assiciated_type(db.into_inner(), &parse_user_header(req)?, "debts")
            .await?,
    ))
}

#[get("/api/tags")]
pub async fn get_tags(req: HttpRequest, db: Data<Client>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        Transaction::get_assiciated_type(db.into_inner(), &parse_user_header(req)?, "tags").await?,
    ))
}
