use chrono::{DateTime, Local, Months};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum State {
    Active,
    Terminated,
    Expired,
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
    pub amount: u32,
    pub fix: bool,
    pub cycle: u32, // in months
    pub kind: PaymentKind,
    pub pod: String,
    pub debts: HashMap<String, u32>,
}

impl Payment {
    pub fn debt_sum(&self) -> u32 {
        self.debts.values().cloned().sum()
    }

    pub fn amount(&self) -> u32 {
        self.amount - self.debt_sum()
    }

    pub fn signed_amount(&self) -> i64 {
        match self.kind {
            PaymentKind::Credit => self.amount() as i64,
            _ => -(self.amount() as i64),
        }
    }

    pub fn signed_monthly_amount(&self) -> i64 {
        self.signed_amount() / (self.cycle as i64)
    }

    pub fn date_of_next_payment(&self) -> DateTime<Local> {
        let current = Local::now();
        let mut result = self.first;
        while result < current {
            result = result + Months::new(self.cycle);
        }
        result
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Contract {
    pub id: Option<String>,
    pub title: String,
    pub partner: String,
    pub start: DateTime<Local>,
    pub state: State,
    pub term: u32,   // in months (Mindestvertragslaufzeit)
    pub notice: u32, // in months (Kündigungsfrist)
    pub management: String,
    pub payments: Vec<Payment>,
    pub attachments: Vec<String>,
}

impl Contract {
    pub fn payment<'a>(&'a self) -> &Payment {
        &self.payments[self.payments.len() - 1]
    }
}

#[cfg(feature = "backend")]
impl fancy_surreal::Databasable for Contract {
    fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }
}
