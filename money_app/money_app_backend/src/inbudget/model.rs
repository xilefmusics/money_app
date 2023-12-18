use super::Inbudget;

use crate::database::Database;
use crate::error::AppError;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// TODO: If surreal version 1.1.0 is available, change the query to object function
#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetData {
    inbudgets: HashMap<String, usize>,
}

pub async fn select(db: &Database, user: &str) -> Result<Vec<Inbudget>, AppError> {
    let mut vec = db
        .query::<GetData>(&format!(
            "SELECT inbudgets FROM transaction WHERE user == \"{}\"",
            user,
        ))
        .await?
        .into_iter()
        .map(|get_data| {
            get_data
                .inbudgets
                .keys()
                .map(|inbudget| inbudget.to_string())
                .collect::<Vec<String>>()
        })
        .flatten()
        .collect::<Vec<String>>();
    vec.sort();
    vec.dedup();
    Ok(vec)
}
