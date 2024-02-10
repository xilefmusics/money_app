use super::{AssociatedTypeDiffValues, QueryParams, Wealth};

use crate::error::AppError;
use crate::transaction::{Filter, TransactionModel};
use fancy_surreal::Client;

use chrono::Local;
use std::sync::Arc;

pub struct History;

impl History {
    pub async fn wealth(
        db: Arc<Client>,
        user: &str,
        query_params: QueryParams,
    ) -> Result<Vec<Wealth>, AppError> {
        Ok(Wealth::history(
            TransactionModel::get(db, user, &Filter::default()).await?,
            Local::now(),
            query_params.year.unwrap_or(0),
            query_params.month.unwrap_or(0),
            query_params.day.unwrap_or(0),
            query_params.len.unwrap_or(1),
        ))
    }

    pub async fn pod(
        db: Arc<Client>,
        user: &str,
        query_params: QueryParams,
    ) -> Result<Vec<AssociatedTypeDiffValues>, AppError> {
        Ok(query_params
            .into_date_iter()
            .into_associated_type_values_iter(
                &TransactionModel::get_associated_type_values(
                    db.clone(),
                    &user,
                    &Filter::default(),
                    "pods",
                )
                .await?,
            )
            .accumulate(
                &TransactionModel::get_associated_type(db, &user, &Filter::default(), "pods")
                    .await?,
            )
            .diff()
            .collect::<Vec<AssociatedTypeDiffValues>>())
    }

    pub async fn budget(
        db: Arc<Client>,
        user: &str,
        query_params: QueryParams,
    ) -> Result<Vec<AssociatedTypeDiffValues>, AppError> {
        Ok(query_params
            .into_date_iter()
            .into_associated_type_values_iter(
                &TransactionModel::get_associated_type_values(
                    db.clone(),
                    &user,
                    &Filter::default(),
                    "budgets",
                )
                .await?,
            )
            .accumulate(
                &TransactionModel::get_associated_type(db, &user, &Filter::default(), "budgets")
                    .await?,
            )
            .diff()
            .collect::<Vec<AssociatedTypeDiffValues>>())
    }

    pub async fn inbudget(
        db: Arc<Client>,
        user: &str,
        query_params: QueryParams,
    ) -> Result<Vec<AssociatedTypeDiffValues>, AppError> {
        Ok(query_params
            .into_date_iter()
            .into_associated_type_values_iter(
                &TransactionModel::get_associated_type_values(
                    db.clone(),
                    &user,
                    &Filter::default(),
                    "inbudgets",
                )
                .await?,
            )
            .accumulate(
                &TransactionModel::get_associated_type(db, &user, &Filter::default(), "inbudgets")
                    .await?,
            )
            .diff()
            .collect::<Vec<AssociatedTypeDiffValues>>())
    }

    pub async fn debt(
        db: Arc<Client>,
        user: &str,
        query_params: QueryParams,
    ) -> Result<Vec<AssociatedTypeDiffValues>, AppError> {
        Ok(query_params
            .into_date_iter()
            .into_associated_type_values_iter(
                &TransactionModel::get_associated_type_values(
                    db.clone(),
                    &user,
                    &Filter::default(),
                    "debts",
                )
                .await?,
            )
            .accumulate(
                &TransactionModel::get_associated_type(db, &user, &Filter::default(), "debts")
                    .await?,
            )
            .diff()
            .collect::<Vec<AssociatedTypeDiffValues>>())
    }
}
