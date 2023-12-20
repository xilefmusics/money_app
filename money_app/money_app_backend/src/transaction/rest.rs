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

#[get("/api/pods")]
pub async fn get_pods(req: HttpRequest, db: Data<Client>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        db.table("transactions")
            .owner(&parse_user_header(req)?)
            .select()?
            .field("content.receiver as item")
            .wrapper_js_map("element.item")
            .wrapper_fn("array::group")
            .wrapper_fn("array::sort")
            .query_direct::<String>()
            .await?
            .into_iter()
            .filter(|pod| pod.len() > 0)
            .collect::<Vec<String>>(),
    ))
}

#[get("/api/budgets")]
pub async fn get_budgets(req: HttpRequest, db: Data<Client>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        db.table("transactions")
            .owner(&parse_user_header(req)?)
            .select()?
            .field("content.budgets as item")
            .wrapper_js_map("Object.keys(element.item)")
            .wrapper_fn("array::group")
            .wrapper_fn("array::sort")
            .query_direct::<String>()
            .await?,
    ))
}

#[get("/api/inbudgets")]
pub async fn get_inbudgets(req: HttpRequest, db: Data<Client>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        db.table("transactions")
            .owner(&parse_user_header(req)?)
            .select()?
            .field("content.inbudgets as item")
            .wrapper_js_map("Object.keys(element.item)")
            .wrapper_fn("array::group")
            .wrapper_fn("array::sort")
            .query_direct::<String>()
            .await?,
    ))
}

#[get("/api/debts")]
pub async fn get_debts(req: HttpRequest, db: Data<Client>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        db.table("transactions")
            .owner(&parse_user_header(req)?)
            .select()?
            .field("content.debts as item")
            .wrapper_js_map("Object.keys(element.item)")
            .wrapper_fn("array::group")
            .wrapper_fn("array::sort")
            .query_direct::<String>()
            .await?,
    ))
}

#[get("/api/tags")]
pub async fn get_tags(req: HttpRequest, db: Data<Client>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        db.table("transactions")
            .owner(&parse_user_header(req)?)
            .select()?
            .field("content.tags as item")
            .wrapper_js_map("Object.keys(element.item)")
            .wrapper_fn("array::group")
            .wrapper_fn("array::sort")
            .query_direct::<String>()
            .await?,
    ))
}
