use crate::database::{id::record2string, Database};
use crate::error::AppError;
use crate::transaction::{Transaction, Type};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use surrealdb::opt::RecordId;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct AddData {
    user: String,
    #[serde(rename = "type")]
    ttype: Type,
    date: DateTime<Local>,
    amount: usize,
    sender: Option<String>,
    receiver: Option<String>,
    budgets: HashMap<String, usize>,
    inbudgets: HashMap<String, usize>,
    debts: HashMap<String, usize>,
    tags: HashMap<String, String>,
    attachment: Option<String>,
}

impl AddData {
    pub fn from_transaction(transaction: Transaction, user: String) -> AddData {
        AddData {
            user: user.clone(),
            ttype: transaction.ttype,
            date: transaction.date,
            amount: transaction.amount,
            sender: transaction.sender,
            receiver: transaction.receiver,
            budgets: transaction.budgets,
            inbudgets: transaction.inbudgets,
            debts: transaction.debts,
            tags: transaction.tags,
            attachment: transaction.attachment,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetData {
    user: String,
    id: RecordId,
    #[serde(rename = "type")]
    ttype: Type,
    date: DateTime<Local>,
    amount: usize,
    sender: Option<String>,
    receiver: Option<String>,
    budgets: HashMap<String, usize>,
    inbudgets: HashMap<String, usize>,
    debts: HashMap<String, usize>,
    tags: HashMap<String, String>,
    attachment: Option<String>,
}

impl GetData {
    pub fn to_transaction(self) -> Transaction {
        Transaction {
            id: Some(record2string(&self.id)),
            ttype: self.ttype,
            date: self.date,
            amount: self.amount,
            sender: self.sender,
            receiver: self.receiver,
            budgets: self.budgets,
            inbudgets: self.inbudgets,
            debts: self.debts,
            tags: self.tags,
            attachment: self.attachment,
        }
    }
}

pub async fn add_transactions(
    db: &Database,
    user: String,
    transactions: Vec<Transaction>,
) -> Result<Vec<Transaction>, AppError> {
    if transactions.len() == 0 {
        return Ok(vec![]);
    }
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
