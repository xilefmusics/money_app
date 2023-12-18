use super::Tag;

use crate::database::Database;
use crate::error::AppError;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// TODO: If surreal version 1.1.0 is available, change the query to object function
#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetData {
    tags: HashMap<String, String>,
}

pub async fn select(db: &Database, user: &str) -> Result<Vec<Tag>, AppError> {
    let mut vec = db
        .query::<GetData>(&format!(
            "SELECT tags FROM transaction WHERE user == \"{}\"",
            user,
        ))
        .await?
        .into_iter()
        .map(|get_data| {
            get_data
                .tags
                .keys()
                .map(|tags| tags.to_string())
                .collect::<Vec<String>>()
        })
        .flatten()
        .collect::<Vec<String>>();
    vec.sort();
    vec.dedup();
    Ok(vec)
}
