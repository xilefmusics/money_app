use super::{QueryParams, Transaction, TransactionModel};

use crate::error::AppError;
use crate::rest::parse_user_header;

use fancy_surreal::Client;

use actix_web::{
    delete, get, post, put, web::Data, web::Json, web::Path, web::Query, HttpRequest, HttpResponse,
};

#[get("/api/transactions")]
pub async fn get(
    req: HttpRequest,
    db: Data<Client<'_>>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        TransactionModel::get(
            db.into_inner(),
            &parse_user_header(&req)?,
            &q.to_filter(),
            true,
        )
        .await?,
    ))
}

#[get("/api/transactions/{id}")]
pub async fn get_id(
    req: HttpRequest,
    db: Data<Client<'_>>,
    id: Path<String>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        TransactionModel::get_one(db.into_inner(), &parse_user_header(&req)?, &id.into_inner())
            .await?,
    ))
}

#[put("/api/transactions")]
pub async fn put(
    req: HttpRequest,
    transactions: Json<Vec<Transaction>>,
    db: Data<Client<'_>>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Created().json(
        TransactionModel::put(
            db.into_inner(),
            &parse_user_header(&req)?,
            transactions.into_inner(),
        )
        .await?,
    ))
}

#[delete("/api/transactions")]
pub async fn delete(
    req: HttpRequest,
    transactions: Json<Vec<Transaction>>,
    db: Data<Client<'_>>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::NoContent().json(
        TransactionModel::delete(
            db.into_inner(),
            &parse_user_header(&req)?,
            transactions.into_inner(),
        )
        .await?,
    ))
}

#[post("/api/transactions")]
pub async fn post(
    req: HttpRequest,
    transactions: Json<Vec<Transaction>>,
    db: Data<Client<'_>>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Created().json(
        TransactionModel::create(
            db.into_inner(),
            &parse_user_header(&req)?,
            transactions.into_inner(),
        )
        .await?,
    ))
}

#[get("/api/pods")]
pub async fn get_pods(
    req: HttpRequest,
    db: Data<Client<'_>>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        TransactionModel::get_associated_type(
            db.into_inner(),
            &parse_user_header(&req)?,
            &q.into_inner().to_filter(),
            "pods",
        )
        .await?,
    ))
}

#[get("/api/budgets")]
pub async fn get_budgets(
    req: HttpRequest,
    db: Data<Client<'_>>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        TransactionModel::get_associated_type(
            db.into_inner(),
            &parse_user_header(&req)?,
            &q.into_inner().to_filter(),
            "budgets",
        )
        .await?,
    ))
}

#[get("/api/inbudgets")]
pub async fn get_inbudgets(
    req: HttpRequest,
    db: Data<Client<'_>>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        TransactionModel::get_associated_type(
            db.into_inner(),
            &parse_user_header(&req)?,
            &q.into_inner().to_filter(),
            "inbudgets",
        )
        .await?,
    ))
}

#[get("/api/debts")]
pub async fn get_debts(
    req: HttpRequest,
    db: Data<Client<'_>>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        TransactionModel::get_associated_type(
            db.into_inner(),
            &parse_user_header(&req)?,
            &q.into_inner().to_filter(),
            "debts",
        )
        .await?,
    ))
}

#[get("/api/tags")]
pub async fn get_tags(
    req: HttpRequest,
    db: Data<Client<'_>>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        TransactionModel::get_associated_type(
            db.into_inner(),
            &parse_user_header(&req)?,
            &q.into_inner().to_filter(),
            "tags",
        )
        .await?,
    ))
}
