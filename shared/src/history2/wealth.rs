use super::ValueDiff;
use crate::itertools::{DateIterator, IntoClusterIterator, IntoPairIterator};
use crate::transaction::Transaction;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::iter::Sum;
use std::ops::Add;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Wealth {
    pub date: DateTime<Local>,
    pub income: ValueDiff<i64>,
    pub out: ValueDiff<i64>,
    pub change: ValueDiff<i64>,
    pub real: ValueDiff<i64>,
    pub debt: ValueDiff<i64>,
    pub sum: ValueDiff<i64>,
}

impl Add for Wealth {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            date: other.date,
            income: self.income + other.income,
            out: self.out + other.out,
            change: self.change + other.change,
            real: self.real + other.real,
            debt: self.debt + other.debt,
            sum: self.sum + other.sum,
        }
    }
}

impl Sum for Wealth {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::default(), |acc, x| acc + x)
    }
}

impl From<Transaction> for Wealth {
    fn from(transaction: Transaction) -> Self {
        Self {
            date: transaction.date,
            income: ValueDiff::value(transaction.income()),
            out: ValueDiff::value(transaction.out()),
            change: ValueDiff::value(transaction.income() - transaction.out()),
            real: ValueDiff::value(transaction.signed_amount() - transaction.signed_debt_sum()),
            debt: ValueDiff::value(transaction.signed_debt_sum()),
            sum: ValueDiff::value(transaction.signed_amount()),
        }
    }
}

impl From<(Vec<Transaction>, Option<DateTime<Local>>)> for Wealth {
    fn from((transactions, date): (Vec<Transaction>, Option<DateTime<Local>>)) -> Self {
        let mut wealth: Self = transactions
            .into_iter()
            .map(|transaction| Self::from(transaction))
            .sum();
        if let Some(date) = date {
            wealth.date = date;
        }
        wealth
    }
}

impl Wealth {
    fn accumulate(mut self, state: &mut Self) -> Option<Self> {
        (*state).real = state.real.clone() + self.real;
        (*state).debt = state.debt.clone() + self.debt;
        (*state).sum = state.sum.clone() + self.sum;
        self.real = state.real.clone();
        self.debt = state.debt.clone();
        self.sum = state.sum.clone();
        Some(self)
    }

    fn shift(mut self, next: &Self) -> Self {
        self.income = next.income.clone();
        self.out = next.out.clone();
        self.change = next.change.clone();
        self
    }

    fn diff(&self, next: &Self) -> Self {
        Self {
            date: self.date,
            income: self.income.diff(&next.income),
            out: self.out.diff(&next.out),
            change: self.change.diff(&next.change),
            real: self.real.diff(&next.real),
            debt: self.debt.diff(&next.debt),
            sum: self.sum.diff(&next.sum),
        }
    }

    pub fn history(
        transactions: Vec<Transaction>,
        date: DateTime<Local>,
        year: u32,
        month: u32,
        day: u64,
        len: u32,
    ) -> Vec<Wealth> {
        transactions
            .into_iter()
            .cluster(
                DateIterator::new(date, year, month, day, len),
                |transaction| transaction.date,
            )
            .map(|(date, transactions)| Wealth::from((transactions, date)))
            .scan(Wealth::default(), |acc, new| new.accumulate(acc))
            .pairs()
            .map(|(current, next)| current.shift(&next.unwrap_or(Self::default())))
            .pairs()
            .map(|(current, next)| current.diff(&next.unwrap_or(current.clone())))
            .collect()
    }
}
