use super::{QueryParams, Wealth};

use crate::error::AppError;
use crate::rest::parse_user_header;
use crate::transaction::{Filter, Transaction};

use fancy_surreal::Client;

use actix_web::{get, web::Data, web::Query, HttpRequest, HttpResponse};

#[get("/api/history/wealth")]
pub async fn get_wealth(
    req: HttpRequest,
    db: Data<Client>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        q.into_inner()
            .into_date_iter()
            .into_transactions_iter(
                &Transaction::get(
                    db.into_inner(),
                    &parse_user_header(req)?,
                    &Filter::default(),
                )
                .await?,
            )
            .into_wealth_iter()
            .into_shift_in_out_iter()
            .collect::<Vec<Wealth>>(),
    ))
}

#[get("/api/history/pods")]
pub async fn get_pods(
    req: HttpRequest,
    db: Data<Client>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        Transaction::get_assiciated_type_values(
            db.into_inner(),
            &parse_user_header(req)?,
            &Filter::default(),
            "pods",
        )
        .await?,
    ))
}

#[get("/api/history/budgets")]
pub async fn get_budgets(
    req: HttpRequest,
    db: Data<Client>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        Transaction::get_assiciated_type_values(
            db.into_inner(),
            &parse_user_header(req)?,
            &Filter::default(),
            "budgets",
        )
        .await?,
    ))
}

#[get("/api/history/inbudgets")]
pub async fn get_inbudgets(
    req: HttpRequest,
    db: Data<Client>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        Transaction::get_assiciated_type_values(
            db.into_inner(),
            &parse_user_header(req)?,
            &Filter::default(),
            "inbudgets",
        )
        .await?,
    ))
}

#[get("/api/history/debts")]
pub async fn get_debts(
    req: HttpRequest,
    db: Data<Client>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        Transaction::get_assiciated_type_values(
            db.into_inner(),
            &parse_user_header(req)?,
            &Filter::default(),
            "debts",
        )
        .await?,
    ))
}
