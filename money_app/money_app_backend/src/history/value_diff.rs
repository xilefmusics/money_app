use serde::{Deserialize, Serialize};
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
