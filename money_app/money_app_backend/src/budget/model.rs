use super::Budget;

use crate::database::Database;
use crate::error::AppError;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// TODO: If surreal version 1.1.0 is available, change the query to object function
#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetData {
    budgets: HashMap<String, usize>,
}

pub async fn select(db: &Database, user: &str) -> Result<Vec<Budget>, AppError> {
    let mut vec = db
        .query::<GetData>(&format!(
            "SELECT budgets FROM transaction WHERE user == \"{}\"",
            user,
        ))
        .await?
        .into_iter()
        .map(|get_data| {
            get_data
                .budgets
                .keys()
                .map(|budget| budget.to_string())
                .collect::<Vec<String>>()
        })
        .flatten()
        .collect::<Vec<String>>();
    vec.sort();
    vec.dedup();
    Ok(vec)
}
