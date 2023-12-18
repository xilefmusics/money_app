use super::Pod;

use crate::database::Database;
use crate::error::AppError;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GetData {
    pod: String,
}

pub async fn select(db: &Database, user: &str) -> Result<Vec<Pod>, AppError> {
    Ok(
        db.query::<GetData>(&format!(
            "array::sort(array::distinct(array::concat((SELECT sender as pod FROM transaction WHERE user == \"{}\"),(SELECT receiver as pod FROM transaction WHERE user == \"{}\"))))",
            user,
            user,
        ))
        .await?
        .into_iter()
        .map(|get_data| get_data.pod)
        .filter(|name| name.len() > 0)
        .collect()
    )
}
