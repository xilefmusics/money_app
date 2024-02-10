use crate::contract::Contract;
use crate::history2::Wealth;

use chrono::{DateTime, Datelike, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ExtrapolationItem {
    pub date: DateTime<Local>,
    pub contract_expenses: u32,
    pub contract_income: u32,
    pub planned_savings: u32,
    pub freely_available: i64,
    pub actual_expenses: u32,
    pub actual_income: u32,
    pub actual_savings: u32,
}

impl From<Wealth> for ExtrapolationItem {
    fn from(item: Wealth) -> Self {
        Self {
            date: item.date,
            contract_expenses: 0,
            contract_income: 0,
            planned_savings: 0,
            freely_available: 0,
            actual_expenses: item.out.value as u32,
            actual_income: item.income.value as u32,
            actual_savings: item.real.diff as u32,
        }
    }
}

impl ExtrapolationItem {
    fn contract_income_expenses(
        contracts: &[Contract],
        date: &DateTime<Local>,
        income: bool,
    ) -> u32 {
        contracts
            .iter()
            .map(|contract| contract.payment.clone())
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
            actual_expenses: 0,
            actual_income: 0,
            actual_savings: 0,
        }
    }

    pub fn equalize_freely_available(&self, new_freely_available: i64) -> Self {
        Self {
            date: self.date.clone(),
            contract_expenses: self.contract_expenses,
            contract_income: self.contract_income,
            planned_savings: (self.planned_savings as i64 + self.freely_available
                - new_freely_available) as u32,
            freely_available: new_freely_available,
            actual_expenses: 0,
            actual_income: 0,
            actual_savings: 0,
        }
    }

    pub fn max_freely_available(&self) -> i64 {
        self.freely_available + self.planned_savings as i64
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Extrapolation {
    pub date: DateTime<Local>,
    pub freely_available: i64,
    pub planned_savings: u32,
    pub normal: Vec<ExtrapolationItem>,
    pub equalized_free_money: Vec<ExtrapolationItem>,
}
