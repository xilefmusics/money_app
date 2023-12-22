use super::Filter;

use crate::error::AppError;

use chrono::{DateTime, Local};
use fancy_surreal::{Client, Databasable};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Type {
    In,
    Out,
    Move,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: Option<String>,
    #[serde(rename = "type")]
    pub ttype: Type,
    pub date: DateTime<Local>,
    pub amount: usize,
    pub sender: Option<String>,
    pub receiver: Option<String>,
    pub budgets: HashMap<String, usize>,
    pub inbudgets: HashMap<String, usize>,
    pub debts: HashMap<String, usize>,
    pub tags: HashMap<String, String>,
    pub attachment: Option<String>,
}

impl Databasable for Transaction {
    fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }
}

impl Transaction {
    pub async fn get<'a>(
        db: Arc<Client>,
        user: &str,
        filter: Filter<'a>,
    ) -> Result<Vec<Self>, AppError> {
        let mut select = db.table("transactions").owner(user).select()?;
        for condition in &filter.conditions() {
            select = select.condition(condition);
        }

        Ok(select.order_by("content.date").query::<Self>().await?)
    }

    pub async fn get_one(db: Arc<Client>, user: &str, id: &str) -> Result<Self, AppError> {
        Ok(db
            .table("transactions")
            .owner(user)
            .select()?
            .id(id)
            .query_one::<Self>()
            .await?)
    }

    pub async fn put(
        db: Arc<Client>,
        user: &str,
        transactions: Vec<Self>,
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
        transactions: Vec<Self>,
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
        transactions: Vec<Self>,
    ) -> Result<Vec<Transaction>, AppError> {
        Ok(db
            .table("transactions")
            .owner(user)
            .create(transactions)
            .await?)
    }

    pub async fn get_assiciated_type<'a>(
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
                .query_direct::<String>()
                .await?
                .into_iter()
                .filter(|pod| pod.len() > 0)
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

    pub fn income(&self) -> i64 {
        match self.ttype {
            Type::In => self.amount as i64,
            Type::Out => 0,
            Type::Move => 0,
        }
    }

    pub fn out(&self) -> i64 {
        match self.ttype {
            Type::In => 0,
            Type::Out => self.amount as i64,
            Type::Move => 0,
        }
    }

    pub fn signed_amount(&self) -> i64 {
        match self.ttype {
            Type::In => self.amount as i64,
            Type::Out => -(self.amount as i64),
            Type::Move => 0,
        }
    }

    pub fn debt_sum(&self) -> i64 {
        self.debts.values().sum::<usize>() as i64
    }

    pub fn signed_debt_sum(&self) -> i64 {
        match self.ttype {
            Type::In => self.debt_sum(),
            Type::Out => -self.debt_sum(),
            Type::Move => 0,
        }
    }
}
