use super::Filter;

use crate::error::AppError;
pub use money_app_shared::transaction::{Transaction, Type};

use chrono::{DateTime, Local};
use fancy_surreal::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AssociatedTypeValues {
    pub date: DateTime<Local>,
    pub data: HashMap<String, i64>,
}

impl AssociatedTypeValues {
    pub fn from_transaction_pod(transaction: Transaction) -> AssociatedTypeValues {
        let mut data = HashMap::<String, i64>::new();
        let amount = transaction.amount as i64;

        if let Some(receiver) = transaction.receiver {
            data.insert(receiver, amount);
        }

        if let Some(sender) = transaction.sender {
            data.insert(sender, -amount);
        }

        AssociatedTypeValues {
            date: transaction.date,
            data,
        }
    }

    pub fn from_transaction_debt(transaction: Transaction) -> AssociatedTypeValues {
        let multiplier = match transaction.ttype {
            Type::In => 1,
            Type::Out => -1,
            Type::Move => 0,
        };

        AssociatedTypeValues {
            date: transaction.date,
            data: transaction
                .debts
                .into_iter()
                .map(|(key, value)| (key, value as i64 * multiplier))
                .collect(),
        }
    }
}

pub struct TransactionModel;

impl TransactionModel {
    pub async fn get<'a>(
        db: Arc<Client>,
        user: &str,
        filter: &Filter<'a>,
    ) -> Result<Vec<Transaction>, AppError> {
        let mut select = db.table("transactions").owner(user).select()?;
        for condition in &filter.conditions() {
            select = select.condition(condition);
        }

        Ok(select
            .order_by("content.date")
            .query::<Transaction>()
            .await?)
    }

    pub async fn get_one(db: Arc<Client>, user: &str, id: &str) -> Result<Transaction, AppError> {
        Ok(db
            .table("transactions")
            .owner(user)
            .select()?
            .id(id)
            .query_one::<Transaction>()
            .await?)
    }

    pub async fn put(
        db: Arc<Client>,
        user: &str,
        transactions: Vec<Transaction>,
    ) -> Result<Vec<Transaction>, AppError> {
        Ok(db
            .table("transactions")
            .owner(user)
            .update(transactions)
            .await?)
    }

    pub async fn delete(
        db: Arc<Client>,
        user: &str,
        transactions: Vec<Transaction>,
    ) -> Result<Vec<Transaction>, AppError> {
        Ok(db
            .table("transactions")
            .owner(user)
            .delete(transactions)
            .await?)
    }

    pub async fn create(
        db: Arc<Client>,
        user: &str,
        transactions: Vec<Transaction>,
    ) -> Result<Vec<Transaction>, AppError> {
        Ok(db
            .table("transactions")
            .owner(user)
            .create(transactions)
            .await?)
    }

    pub async fn get_associated_type<'a>(
        db: Arc<Client>,
        user: &str,
        filter: &Filter<'a>,
        associated_type: &str,
    ) -> Result<Vec<String>, AppError> {
        let mut select = db.table("transactions").owner(user).select()?;
        for condition in &filter.conditions() {
            select = select.condition(condition);
        }

        Ok(if associated_type == "pods" {
            select
                .field("content.receiver as item")
                .wrapper_js_map("element.item")
                .wrapper_fn("array::group")
                .wrapper_fn("array::sort")
                .query_direct::<Option<String>>()
                .await?
                .into_iter()
                .filter(|opod| match opod {
                    Some(pod) => pod.len() > 0,
                    None => false,
                })
                .map(|pod| pod.unwrap())
                .collect::<Vec<String>>()
        } else {
            select
                .field(&format!("content.{} as item", associated_type))
                .wrapper_js_map("Object.keys(element.item)")
                .wrapper_fn("array::group")
                .wrapper_fn("array::sort")
                .query_direct::<String>()
                .await?
        })
    }

    pub async fn get_associated_type_values<'a>(
        db: Arc<Client>,
        user: &str,
        filter: &Filter<'a>,
        associated_type: &str,
    ) -> Result<Vec<AssociatedTypeValues>, AppError> {
        let mut select = db
            .table("transactions")
            .owner(user)
            .select()?
            .order_by("date");
        for condition in &filter.conditions() {
            select = select.condition(condition);
        }

        Ok(if associated_type == "pods" {
            TransactionModel::get(db, user, filter)
                .await?
                .into_iter()
                .map(|t| AssociatedTypeValues::from_transaction_pod(t))
                .collect()
        } else if associated_type == "debts" {
            TransactionModel::get(db, user, filter)
                .await?
                .into_iter()
                .map(|t| AssociatedTypeValues::from_transaction_debt(t))
                .collect()
        } else {
            select
                .field("content.date as date")
                .field(&format!("content.{} as data", associated_type))
                .query_direct::<AssociatedTypeValues>()
                .await?
        })
    }
}
