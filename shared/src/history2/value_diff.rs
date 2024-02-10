use serde::{Deserialize, Serialize};
use std::ops::{Add, Sub};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ValueDiff<T>
where
    T: Default,
{
    pub value: T,
    pub diff: T,
}

impl<T> ValueDiff<T>
where
    T: Default,
{
    pub fn new(value: T, diff: T) -> Self {
        Self { value, diff }
    }

    pub fn value(value: T) -> Self {
        Self {
            value,
            diff: T::default(),
        }
    }
}

impl<T> Add for ValueDiff<T>
where
    T: Add<Output = T> + Default,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            value: self.value + other.value,
            diff: self.diff + other.diff,
        }
    }
}

impl<T> ValueDiff<T>
where
    T: Add + Default + Sub<Output = T> + Clone,
{
    pub fn diff(&self, other: &Self) -> Self {
        Self {
            value: self.value.clone(),
            diff: other.value.clone() - self.value.clone(),
        }
    }
}
