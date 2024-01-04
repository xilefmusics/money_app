use super::{AssociatedTypeDiffValues, QueryParams, Wealth};

use crate::error::AppError;
use crate::transaction::{Filter, Transaction};
use fancy_surreal::Client;

use std::sync::Arc;

pub struct History;

impl History {
    pub async fn wealth(
        db: Arc<Client>,
        user: &str,
        query_params: QueryParams,
    ) -> Result<Vec<Wealth>, AppError> {
        Ok(query_params
            .into_date_iter()
            .into_transactions_iter(&Transaction::get(db, user, &Filter::default()).await?)
            .into_wealth_iter()
            .into_shift_in_out_iter()
            .collect::<Vec<Wealth>>())
    }

    pub async fn pod(
        db: Arc<Client>,
        user: &str,
        query_params: QueryParams,
    ) -> Result<Vec<AssociatedTypeDiffValues>, AppError> {
        Ok(query_params
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
            .collect::<Vec<AssociatedTypeDiffValues>>())
    }
}
