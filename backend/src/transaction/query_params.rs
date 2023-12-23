use super::Filter;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryParams {
    pub year: Option<i32>,
    pub month: Option<u32>,
    pub pod: Option<String>,
    pub debt: Option<String>,
    pub budget: Option<String>,
    pub inbudget: Option<String>,
    #[serde(rename = "type")]
    pub ttype: Option<String>,
}

impl QueryParams {
    pub fn to_filter<'a>(&'a self) -> Filter<'a> {
        Filter::new(
            self.year,
            self.month,
            self.pod.as_deref(),
            self.debt.as_deref(),
            self.budget.as_deref(),
            self.inbudget.as_deref(),
            self.ttype.as_deref(),
        )
    }
}
