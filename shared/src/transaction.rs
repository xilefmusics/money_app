use chrono::{DateTime, Local};
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

impl Transaction {
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

    pub fn title(&self) -> String {
        match self.ttype {
            Type::In => self.receiver.clone().unwrap(),
            Type::Out => self.sender.clone().unwrap(),
            Type::Move => format!(
                "{} to {}",
                self.sender.clone().unwrap(),
                self.receiver.clone().unwrap()
            ),
        }
    }
}

#[cfg(feature = "backend")]
impl fancy_surreal::Databasable for Transaction {
    fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }
}
