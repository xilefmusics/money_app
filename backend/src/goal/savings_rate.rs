use super::{Goal, GoalData, GoalModel};

use crate::error::AppError;
use crate::history::{History, QueryParams};
use fancy_surreal::Client;

use chrono::{DateTime, Datelike, Local};
use std::sync::Arc;

pub fn savings_rate_pure(
    goals: &[Goal],
    current_wealth: i64,
    current_date: DateTime<Local>,
) -> u32 {
    let max = goals
        .iter()
        .map(|goal| {
            let months = goal.due.month() as i64 - current_date.month() as i64
                + (goal.due.year() as i64 - current_date.year() as i64) * 12;
            match goal.data {
                GoalData::RealWealth(goal) => (goal as i64 - current_wealth) / months,
            }
        })
        .max()
        .unwrap_or(0);
    if max > 0 {
        max as u32
    } else {
        0
    }
}

pub async fn savings_rate(db: Arc<Client>, user: &str) -> Result<u32, AppError> {
    let goals = GoalModel::get(db.clone(), user).await?;
    let current_wealth = History::wealth(db, user, QueryParams::default()).await?[0]
        .real
        .value;
    let current_date = Local::now();
    Ok(savings_rate_pure(&goals, current_wealth, current_date))
}
