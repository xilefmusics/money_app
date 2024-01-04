use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::iter::Sum;
use std::ops::Add;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ValueDiff {
    pub value: i64,
    pub diff: i64,
}

impl ValueDiff {
    pub fn new(value: i64, diff: i64) -> Self {
        Self { value, diff }
    }
}

impl Add for ValueDiff {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
            diff: self.diff + other.diff,
        }
    }
}

impl Into<ValueDiff> for i64 {
    fn into(self) -> ValueDiff {
        ValueDiff {
            value: self,
            diff: 0,
        }
    }
}

pub trait Diff {
    fn diff(&self, other: &Self) -> Self;
}

impl Diff for ValueDiff {
    fn diff(&self, other: &Self) -> Self {
        Self {
            value: self.value,
            diff: self.value - other.value,
        }
    }
}

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
