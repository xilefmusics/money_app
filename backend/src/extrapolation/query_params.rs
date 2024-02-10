use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryParams {
    pub year: Option<i32>,
    pub month: Option<u32>,
}
