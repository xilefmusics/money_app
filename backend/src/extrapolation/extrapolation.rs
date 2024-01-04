use super::DateIterator;

use crate::contract::Contract;
pub use money_app_shared::exploration_item::ExtrapolationItem;

pub fn create_extrapolation(contracts: &[Contract], savings_rate: u32) -> Vec<ExtrapolationItem> {
    DateIterator::new()
        .map(|date| ExtrapolationItem::new(contracts, savings_rate, date))
        .collect()
}
