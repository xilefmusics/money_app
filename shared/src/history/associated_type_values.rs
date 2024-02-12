use super::ValueDiff;
use crate::itertools::{DateIterator, IntoClusterIterator, IntoPairIterator};
use crate::transaction::AssociatedTypeValues as AssociatedTypeValuesT;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::iter::Sum;
use std::ops::Add;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AssociatedTypeValues {
    pub date: DateTime<Local>,
    pub data: HashMap<String, ValueDiff<i64>>,
}

impl Add for AssociatedTypeValues {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut data = HashMap::new();
        for (key, value) in self.data.iter().chain(other.data.iter()) {
            let counter = data.entry(key.clone()).or_insert(ValueDiff::default());
            *counter = counter.clone() + value.clone();
        }
        Self {
            date: other.date,
            data,
        }
    }
}

impl From<AssociatedTypeValuesT> for AssociatedTypeValues {
    fn from(other: AssociatedTypeValuesT) -> Self {
        Self {
            date: other.date,
            data: other
                .data
                .into_iter()
                .map(|(key, value)| (key, ValueDiff::value(value)))
                .collect(),
        }
    }
}

impl Sum for AssociatedTypeValues {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Self::default(), |acc, x| acc + x)
    }
}

impl From<(Vec<AssociatedTypeValuesT>, Option<DateTime<Local>>)> for AssociatedTypeValues {
    fn from((values, date): (Vec<AssociatedTypeValuesT>, Option<DateTime<Local>>)) -> Self {
        let mut out_values: Self = values.into_iter().map(|values| Self::from(values)).sum();
        if let Some(date) = date {
            out_values.date = date;
        }
        out_values
    }
}

impl AssociatedTypeValues {
    fn accumulate(self, state: &mut Self) -> Option<Self> {
        (*state) = state.clone() + self;
        Some(state.clone())
    }

    fn diff(&self, next: &Self) -> Self {
        let mut data = HashMap::new();
        for (key, value) in &next.data {
            if let Some(value2) = self.data.get(key) {
                data.insert(key.clone(), value.diff(value2));
            }
        }
        Self {
            date: self.date,
            data,
        }
    }

    pub fn history(
        values: Vec<AssociatedTypeValuesT>,
        date: DateTime<Local>,
        year: u32,
        month: u32,
        day: u64,
        len: u32,
    ) -> Vec<Self> {
        values
            .into_iter()
            .cluster(DateIterator::new(date, year, month, day, len), |value| {
                value.date
            })
            .map(|(date, values)| Self::from((values, date)))
            .scan(Self::default(), |acc, new| new.accumulate(acc))
            .pairs()
            .map(|(current, next)| current.diff(&next.unwrap_or(current.clone())))
            .collect()
    }
}
