use super::transaction::AssociatedTypeValues;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AssociatedTypeDiffValues {
    pub date: DateTime<Local>,
    pub data: HashMap<String, ValueDiff>,
}

impl AssociatedTypeDiffValues {
    pub fn from_one(other: AssociatedTypeValues) -> Self {
        Self {
            date: other.date,
            data: other
                .data
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect::<HashMap<String, ValueDiff>>(),
        }
    }

    pub fn from_two(first: AssociatedTypeValues, second: AssociatedTypeValues) -> Self {
        Self {
            date: first.date,
            data: first
                .data
                .into_iter()
                .map(|(k, v)| {
                    (
                        k.clone(),
                        (ValueDiff::new(v, *second.data.get(&k).unwrap_or(&0) - v)).into(),
                    )
                })
                .collect::<HashMap<String, ValueDiff>>(),
        }
    }
}
