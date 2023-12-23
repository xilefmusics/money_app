use super::{AssociatedTypeDiffValues, QueryParams, Wealth};

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
    let user = parse_user_header(req)?;
    let db = db.into_inner();
    Ok(HttpResponse::Ok().json(
        q.into_inner()
            .into_date_iter()
            .into_associated_type_values_iter(
                &Transaction::get_associated_type_values(
                    db.clone(),
                    &user,
                    &Filter::default(),
                    "pods",
                )
                .await?,
            )
            .accumulate(
                &Transaction::get_associated_type(db, &user, &Filter::default(), "pods").await?,
            )
            .diff()
            .collect::<Vec<AssociatedTypeDiffValues>>(),
    ))
}

#[get("/api/history/budgets")]
pub async fn get_budgets(
    req: HttpRequest,
    db: Data<Client>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    let user = parse_user_header(req)?;
    let db = db.into_inner();
    Ok(HttpResponse::Ok().json(
        q.into_inner()
            .into_date_iter()
            .into_associated_type_values_iter(
                &Transaction::get_associated_type_values(
                    db.clone(),
                    &user,
                    &Filter::default(),
                    "budgets",
                )
                .await?,
            )
            .accumulate(
                &Transaction::get_associated_type(db, &user, &Filter::default(), "budgets").await?,
            )
            .diff()
            .collect::<Vec<AssociatedTypeDiffValues>>(),
    ))
}

#[get("/api/history/inbudgets")]
pub async fn get_inbudgets(
    req: HttpRequest,
    db: Data<Client>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    let user = parse_user_header(req)?;
    let db = db.into_inner();
    Ok(HttpResponse::Ok().json(
        q.into_inner()
            .into_date_iter()
            .into_associated_type_values_iter(
                &Transaction::get_associated_type_values(
                    db.clone(),
                    &user,
                    &Filter::default(),
                    "inbudgets",
                )
                .await?,
            )
            .accumulate(
                &Transaction::get_associated_type(db, &user, &Filter::default(), "inbudgets")
                    .await?,
            )
            .diff()
            .collect::<Vec<AssociatedTypeDiffValues>>(),
    ))
}

#[get("/api/history/debts")]
pub async fn get_debts(
    req: HttpRequest,
    db: Data<Client>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    let user = parse_user_header(req)?;
    let db = db.into_inner();
    Ok(HttpResponse::Ok().json(
        q.into_inner()
            .into_date_iter()
            .into_associated_type_values_iter(
                &Transaction::get_associated_type_values(
                    db.clone(),
                    &user,
                    &Filter::default(),
                    "debts",
                )
                .await?,
            )
            .accumulate(
                &Transaction::get_associated_type(db, &user, &Filter::default(), "debts").await?,
            )
            .diff()
            .collect::<Vec<AssociatedTypeDiffValues>>(),
    ))
}
