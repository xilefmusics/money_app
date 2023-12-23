use super::{DateIterator, Diff, ValueDiff};

use crate::transaction::AssociatedTypeValues;

use std::collections::HashMap;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

pub struct AssociatedTypeValuesIterator<'a> {
    date_iterator: DateIterator,
    associated_type_values: &'a [AssociatedTypeValues],
}

impl<'a> AssociatedTypeValuesIterator<'a> {
    pub fn new(
        date_iterator: DateIterator,
        associated_type_values: &'a [AssociatedTypeValues],
    ) -> Self {
        Self {
            date_iterator,
            associated_type_values,
        }
    }

    pub fn accumulate(self, keys: &'a [String]) -> AssociatedTypeValuesAccumulatorIterator<'a> {
        AssociatedTypeValuesAccumulatorIterator::new(self, keys)
    }
}

impl<'a> Iterator for AssociatedTypeValuesIterator<'a> {
    type Item = (&'a [AssociatedTypeValues], DateTime<Local>);

    fn next(&mut self) -> Option<Self::Item> {
        let date = self.date_iterator.next()?;

        let (associated_type_values, rest) = self.associated_type_values.split_at(
            self.associated_type_values
                .iter()
                .position(|associated_type_values| associated_type_values.date > date)
                .unwrap_or(self.associated_type_values.len()),
        );
        self.associated_type_values = rest;

        Some((associated_type_values, date))
    }
}

pub struct AssociatedTypeValuesAccumulatorIterator<'a> {
    associated_type_values_iterator: AssociatedTypeValuesIterator<'a>,
    current: AssociatedTypeValues,
}

impl<'a> AssociatedTypeValuesAccumulatorIterator<'a> {
    pub fn new(
        associated_type_values_iterator: AssociatedTypeValuesIterator<'a>,
        keys: &'a [String],
    ) -> Self {
        Self {
            associated_type_values_iterator,
            current: AssociatedTypeValues {
                date: DateTime::default(),
                data: keys
                    .into_iter()
                    .map(|k| (k.clone(), 0))
                    .collect::<HashMap<String, i64>>(),
            },
        }
    }

    pub fn diff(self) -> AssociatedTypeValuesDifferIterator<'a> {
        AssociatedTypeValuesDifferIterator::new(self)
    }
}

impl<'a> Iterator for AssociatedTypeValuesAccumulatorIterator<'a> {
    type Item = AssociatedTypeValues;

    fn next(&mut self) -> Option<Self::Item> {
        let (associated_type_values_slice, date) = self.associated_type_values_iterator.next()?;

        for associated_type_value in associated_type_values_slice {
            for (key, value) in associated_type_value.data.iter() {
                self.current
                    .data
                    .entry(key.clone())
                    .and_modify(|v| *v = *v + value);
            }
        }

        self.current.date = date;
        Some(self.current.clone())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AssociatedTypeDiffValues {
    pub date: DateTime<Local>,
    pub data: HashMap<String, ValueDiff>,
}

impl AssociatedTypeDiffValues {
    fn from_one(other: AssociatedTypeValues) -> Self {
        Self {
            date: other.date,
            data: other
                .data
                .into_iter()
                .map(|(k, v)| (k, v.into()))
                .collect::<HashMap<String, ValueDiff>>(),
        }
    }

    fn from_two(first: AssociatedTypeValues, second: AssociatedTypeValues) -> Self {
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

pub struct AssociatedTypeValuesDifferIterator<'a> {
    associated_type_values_accumulater_iterator: AssociatedTypeValuesAccumulatorIterator<'a>,
    last: Option<AssociatedTypeValues>,
}

impl<'a> AssociatedTypeValuesDifferIterator<'a> {
    pub fn new(
        mut associated_type_values_accumulater_iterator: AssociatedTypeValuesAccumulatorIterator<
            'a,
        >,
    ) -> Self {
        let last = associated_type_values_accumulater_iterator.next();
        Self {
            associated_type_values_accumulater_iterator,
            last,
        }
    }
}

impl<'a> Iterator for AssociatedTypeValuesDifferIterator<'a> {
    type Item = AssociatedTypeDiffValues;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.associated_type_values_accumulater_iterator.next();
        if next.is_none() {
            return Some(AssociatedTypeDiffValues::from_one(self.last.take()?));
        }
        let last = self.last.take().unwrap();
        self.last = next;

        Some(AssociatedTypeDiffValues::from_two(
            last,
            self.last.clone().unwrap(),
        ))
    }
}
