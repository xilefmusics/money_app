use crate::database::{id::record2string, Database};
use crate::error::AppError;
use crate::transaction::{Amount, Inner, Transaction};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use surrealdb::opt::RecordId;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AddData {
    user: String,
    date: DateTime<Local>,
    amount: Amount,
    inner: Inner,
    attachment: Option<String>,
    tags: HashMap<String, String>,
}

impl AddData {
    pub fn from_transaction(transaction: Transaction, user: String) -> AddData {
        AddData {
            user: user.clone(),
            date: transaction.date,
            amount: transaction.amount,
            inner: transaction.inner,
            attachment: transaction.attachment,
            tags: transaction.tags,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetData {
    id: RecordId,
    user: String,
    date: DateTime<Local>,
    amount: Amount,
    inner: Inner,
    attachment: Option<String>,
    tags: HashMap<String, String>,
}

impl GetData {
    pub fn to_transaction(self) -> Transaction {
        Transaction {
            id: Some(record2string(&self.id)),
            date: self.date,
            amount: self.amount,
            inner: self.inner,
            attachment: self.attachment,
            tags: self.tags,
        }
    }
}

pub async fn add_transactions(
    db: &Database,
    user: String,
    transactions: Vec<Transaction>,
) -> Result<Vec<Transaction>, AppError> {
    Ok(db
        .create::<AddData, GetData>(
            "transaction",
            transactions
                .into_iter()
                .map(|transaction| AddData::from_transaction(transaction, user.clone()))
                .collect::<Vec<AddData>>(),
        )
        .await?
        .into_iter()
        .map(|get_data| get_data.to_transaction())
        .collect())
}
