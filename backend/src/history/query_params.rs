use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct QueryParams {
    pub year: Option<u32>,
    pub month: Option<u32>,
    pub day: Option<u64>,
    pub len: Option<u32>,
}
