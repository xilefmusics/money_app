use super::{AssociatedTypeValues, QueryParams, Wealth};

use crate::error::AppError;
use crate::transaction::{Filter, TransactionModel};
use fancy_surreal::Client;

use chrono::Local;
use std::sync::Arc;

pub struct History;

impl History {
    pub async fn wealth(
        db: Arc<Client<'_>>,
        user: &str,
        query_params: QueryParams,
    ) -> Result<Vec<Wealth>, AppError> {
        Ok(Wealth::history(
            TransactionModel::get(
                db,
                user,
                &Filter::default().start_option(query_params.start),
                false,
            )
            .await?,
            query_params.end.unwrap_or(Local::now()),
            query_params.year.unwrap_or(0),
            query_params.month.unwrap_or(0),
            query_params.day.unwrap_or(0),
            query_params.len.unwrap_or(1),
        ))
    }

    async fn associated_type_values(
        db: Arc<Client<'_>>,
        user: &str,
        query_params: QueryParams,
        t: &str,
    ) -> Result<Vec<AssociatedTypeValues>, AppError> {
        Ok(AssociatedTypeValues::history(
            TransactionModel::get_associated_type_values(
                db.clone(),
                &user,
                &Filter::default().start_option(query_params.start),
                t,
            )
            .await?,
            query_params.end.unwrap_or(Local::now()),
            query_params.year.unwrap_or(0),
            query_params.month.unwrap_or(0),
            query_params.day.unwrap_or(0),
            query_params.len.unwrap_or(1),
        ))
    }

    pub async fn pod(
        db: Arc<Client<'_>>,
        user: &str,
        query_params: QueryParams,
    ) -> Result<Vec<AssociatedTypeValues>, AppError> {
        Self::associated_type_values(db, user, query_params, "pods").await
    }

    pub async fn budget(
        db: Arc<Client<'_>>,
        user: &str,
        query_params: QueryParams,
    ) -> Result<Vec<AssociatedTypeValues>, AppError> {
        Self::associated_type_values(db, user, query_params, "budgets").await
    }

    pub async fn inbudget(
        db: Arc<Client<'_>>,
        user: &str,
        query_params: QueryParams,
    ) -> Result<Vec<AssociatedTypeValues>, AppError> {
        Self::associated_type_values(db, user, query_params, "inbudgets").await
    }

    pub async fn debt(
        db: Arc<Client<'_>>,
        user: &str,
        query_params: QueryParams,
    ) -> Result<Vec<AssociatedTypeValues>, AppError> {
        Self::associated_type_values(db, user, query_params, "debts").await
    }
}
