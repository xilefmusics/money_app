use crate::AppError;

use fancy_surreal::{Client, Databasable};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum State {
    Active,
    Terminated,
    Expired,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PaymentCycle {
    Monthly,
    Quarterly,
    Daily,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum PaymentKind {
    Active,
    Debit,
    Credit,
    PayPal,
    GooglePay,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Payment {
    pub first: DateTime<Local>,
    pub amount: usize,
    pub fix: bool,
    pub cycle: PaymentCycle,
    pub kind: PaymentKind,
    pub pod: String,
    pub debts: HashMap<String, usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contract {
    pub id: Option<String>,
    pub title: String,
    pub partner: String,
    pub start: DateTime<Local>,
    pub state: State,
    pub term: usize,   // in months (Mindestvertragslaufzeit)
    pub notice: usize, // in months (Kündigungsfrist)
    pub management: String,
    pub payments: Vec<Payment>,
    pub attachments: Vec<String>,
}

impl Databasable for Contract {
    fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }
}

impl Contract {
    pub async fn get<'a>(db: Arc<Client>, user: &str) -> Result<Vec<Self>, AppError> {
        Ok(db
            .table("contracts")
            .owner(user)
            .select()?
            .query::<Self>()
            .await?)
    }

    pub async fn get_one(db: Arc<Client>, user: &str, id: &str) -> Result<Self, AppError> {
        Ok(db
            .table("contracts")
            .owner(user)
            .select()?
            .id(id)
            .query_one::<Self>()
            .await?)
    }

    pub async fn put(
        db: Arc<Client>,
        user: &str,
        contracts: Vec<Self>,
    ) -> Result<Vec<Self>, AppError> {
        Ok(db.table("contracts").owner(user).update(contracts).await?)
    }

    pub async fn delete(
        db: Arc<Client>,
        user: &str,
        contracts: Vec<Self>,
    ) -> Result<Vec<Contract>, AppError> {
        Ok(db.table("contracts").owner(user).delete(contracts).await?)
    }

    pub async fn create(
        db: Arc<Client>,
        user: &str,
        contracts: Vec<Self>,
    ) -> Result<Vec<Self>, AppError> {
        Ok(db.table("contracts").owner(user).create(contracts).await?)
    }
}
