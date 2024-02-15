use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct QueryParams {
    pub year: Option<u32>,
    pub month: Option<u32>,
    pub day: Option<u64>,
    pub len: Option<u32>,
    #[serde(default, with = "crate::serde_custom::yyyy_mm_dd_option")]
    pub start: Option<DateTime<Local>>,
    #[serde(default, with = "crate::serde_custom::yyyy_mm_dd_option")]
    pub end: Option<DateTime<Local>>,
}
