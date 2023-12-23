use super::DateIterator;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryParams {
    pub year: Option<u32>,
    pub month: Option<u32>,
    pub day: Option<u64>,
    pub len: Option<u32>,
}

impl QueryParams {
    pub fn into_date_iter(self) -> DateIterator {
        DateIterator::new(
            self.year.unwrap_or(0),
            self.month.unwrap_or(0),
            self.day.unwrap_or(0),
            self.len.unwrap_or(1),
        )
    }
}
