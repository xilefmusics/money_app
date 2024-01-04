use crate::contract::Contract;

use chrono::{DateTime, Datelike, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ExtrapolationItem {
    pub date: DateTime<Local>,
    pub contract_expenses: u32,
    pub contract_income: u32,
    pub planned_savings: u32,
    pub freely_available: i64,
}

impl ExtrapolationItem {
    fn contract_income_expenses(
        contracts: &[Contract],
        date: &DateTime<Local>,
        income: bool,
    ) -> u32 {
        contracts
            .iter()
            .map(|contract| contract.payment())
            .filter(|payment| payment.has_income() == income)
            .filter(|payment| payment.first.month() % payment.cycle == date.month() % payment.cycle)
            .map(|payment| payment.amount())
            .sum()
    }

    pub fn new(contracts: &[Contract], savings_rate: u32, date: DateTime<Local>) -> Self {
        let contract_expenses = Self::contract_income_expenses(contracts, &date, false);
        let contract_income = Self::contract_income_expenses(contracts, &date, true);
        let planned_savings = savings_rate;
        ExtrapolationItem {
            date,
            contract_expenses,
            contract_income,
            planned_savings,
            freely_available: (contract_income as i64)
                - ((contract_expenses + planned_savings) as i64),
        }
    }
}
