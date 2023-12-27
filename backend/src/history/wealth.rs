use super::{Diff, ValueDiff};

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::iter::Sum;
use std::ops::Add;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Wealth {
    pub date: DateTime<Local>,
    #[serde(rename = "in")]
    pub income: ValueDiff,
    pub out: ValueDiff,
    pub change: ValueDiff,
    pub real: ValueDiff,
    pub debt: ValueDiff,
    pub sum: ValueDiff,
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

impl Diff for Wealth {
    fn diff(&self, other: &Self) -> Self {
        Self {
            date: self.date,
            income: self.income.diff(&other.income),
            out: self.out.diff(&other.out),
            real: self.real.diff(&other.real),
            change: self.change.diff(&other.change),
            debt: self.debt.diff(&other.debt),
            sum: self.sum.diff(&other.sum),
        }
    }
}
