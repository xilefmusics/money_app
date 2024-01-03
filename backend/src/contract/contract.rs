use crate::AppError;

use fancy_surreal::Client;
pub use money_app_shared::contract::Contract;

use std::sync::Arc;

pub struct ContractModel;

impl ContractModel {
    pub async fn get<'a>(db: Arc<Client>, user: &str) -> Result<Vec<Contract>, AppError> {
        Ok(db
            .table("contracts")
            .owner(user)
            .select()?
            .query::<Contract>()
            .await?)
    }

    pub async fn get_one(db: Arc<Client>, user: &str, id: &str) -> Result<Contract, AppError> {
        Ok(db
            .table("contracts")
            .owner(user)
            .select()?
            .id(id)
            .query_one::<Contract>()
            .await?)
    }

    pub async fn put(
        db: Arc<Client>,
        user: &str,
        contracts: Vec<Contract>,
    ) -> Result<Vec<Contract>, AppError> {
        Ok(db.table("contracts").owner(user).update(contracts).await?)
    }

    pub async fn delete(
        db: Arc<Client>,
        user: &str,
        contracts: Vec<Contract>,
    ) -> Result<Vec<Contract>, AppError> {
        Ok(db.table("contracts").owner(user).delete(contracts).await?)
    }

    pub async fn create(
        db: Arc<Client>,
        user: &str,
        contracts: Vec<Contract>,
    ) -> Result<Vec<Contract>, AppError> {
        Ok(db.table("contracts").owner(user).create(contracts).await?)
    }
}
