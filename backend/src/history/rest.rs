use super::{History, QueryParams};

use crate::error::AppError;
use crate::rest::parse_user_header;

use fancy_surreal::Client;

use actix_web::{get, web::Data, web::Query, HttpRequest, HttpResponse};

#[get("/api/history/wealth")]
pub async fn get_wealth(
    req: HttpRequest,
    db: Data<Client<'_>>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok()
        .json(History::wealth(db.into_inner(), &parse_user_header(&req)?, q.into_inner()).await?))
}

#[get("/api/history/pods")]
pub async fn get_pods(
    req: HttpRequest,
    db: Data<Client<'_>>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok()
        .json(History::pod(db.into_inner(), &parse_user_header(&req)?, q.into_inner()).await?))
}

#[get("/api/history/budgets")]
pub async fn get_budgets(
    req: HttpRequest,
    db: Data<Client<'_>>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok()
        .json(History::budget(db.into_inner(), &parse_user_header(&req)?, q.into_inner()).await?))
}

#[get("/api/history/inbudgets")]
pub async fn get_inbudgets(
    req: HttpRequest,
    db: Data<Client<'_>>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok()
        .json(History::inbudget(db.into_inner(), &parse_user_header(&req)?, q.into_inner()).await?))
}

#[get("/api/history/debts")]
pub async fn get_debts(
    req: HttpRequest,
    db: Data<Client<'_>>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok()
        .json(History::debt(db.into_inner(), &parse_user_header(&req)?, q.into_inner()).await?))
}
