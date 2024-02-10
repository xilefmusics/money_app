use super::DateIterator;

use crate::contract::{Contract, ContractModel};
use crate::goal::savings_rate;
use crate::history::{History, QueryParams};
use crate::AppError;
use fancy_surreal::Client;
pub use money_app_shared::extrapolation::{Extrapolation, ExtrapolationItem};
use money_app_shared::history2::Wealth;

use std::sync::Arc;

pub fn create_extrapolation_pure(
    contracts: &[Contract],
    savings_rate: u32,
    year: i32,
    month: u32,
    history: Vec<Wealth>,
) -> Extrapolation {
    let normal = history
        .into_iter()
        .map(|item| ExtrapolationItem::from(item))
        .chain(
            DateIterator::new(year, month)
                .map(|date| ExtrapolationItem::new(contracts, savings_rate, date)),
        )
        .collect::<Vec<ExtrapolationItem>>();

    let freely_available = normal.iter().map(|item| item.freely_available).sum::<i64>();
    let planned_savings = normal.iter().map(|item| item.planned_savings).sum::<u32>();

    let equalized_free_money = {
        let freely_available = std::cmp::min(
            freely_available / 12,
            normal
                .iter()
                .map(|item| item.max_freely_available())
                .min()
                .unwrap_or(0),
        );
        normal
            .iter()
            .map(|item| item.equalize_freely_available(freely_available))
            .collect::<Vec<ExtrapolationItem>>()
    };

    Extrapolation {
        date: normal[0].date.clone(),
        freely_available,
        planned_savings,
        normal,
        equalized_free_money,
    }
}

pub async fn create_extrapolation(
    db: Arc<Client>,
    user: &str,
    year: i32,
    month: u32,
) -> Result<Extrapolation, AppError> {
    let wealth = History::wealth(
        db.clone(),
        user,
        QueryParams {
            year: None,
            month: Some(1),
            day: None,
            len: Some(month - 1),
        },
    )
    .await?;
    let contracts = ContractModel::get(db.clone(), user).await?;
    let savings_rate = savings_rate(db, user).await?;
    Ok(create_extrapolation_pure(
        &contracts,
        savings_rate,
        year,
        month,
        wealth,
    ))
}
