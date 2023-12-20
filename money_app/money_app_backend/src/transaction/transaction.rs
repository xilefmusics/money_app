use chrono::{DateTime, Local};
use fancy_surreal::Databasable;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
