use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Amount {
    amount: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InOut {
    pod: String,
    budget: HashMap<String, Amount>,
    debts: HashMap<String, Amount>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Move {
    from: String,
    to: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Inner {
    In(InOut),
    Out(InOut),
    Move(Move),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: Option<String>,
    pub date: DateTime<Local>,
    pub amount: Amount,
    pub inner: Inner,
    pub attachment: Option<String>,
    pub tags: HashMap<String, String>,
}
