use super::DateIterator;

use crate::contract::{Contract, ContractModel};
use crate::goal::savings_rate;
use crate::AppError;
use fancy_surreal::Client;
pub use money_app_shared::exploration_item::ExtrapolationItem;

use std::sync::Arc;

pub fn create_extrapolation_pure(
    contracts: &[Contract],
    savings_rate: u32,
) -> Vec<ExtrapolationItem> {
    DateIterator::new()
        .map(|date| ExtrapolationItem::new(contracts, savings_rate, date))
        .collect()
}

pub async fn create_extrapolation(
    db: Arc<Client>,
    user: &str,
) -> Result<Vec<ExtrapolationItem>, AppError> {
    let contracts = ContractModel::get(db.clone(), user).await?;
    let savings_rate = savings_rate(db, user).await?;
    Ok(create_extrapolation_pure(&contracts, savings_rate))
}
