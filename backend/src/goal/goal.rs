use crate::AppError;

use fancy_surreal::Client;
pub use money_app_shared::goal::{Goal, GoalData};

use std::sync::Arc;

pub struct GoalModel;

impl GoalModel {
    pub async fn get<'a>(db: Arc<Client<'_>>, user: &str) -> Result<Vec<Goal>, AppError> {
        Ok(db
            .table("goals")
            .owner(user)
            .select()?
            .query::<Goal>()
            .await?)
    }

    pub async fn get_one(db: Arc<Client<'_>>, user: &str, id: &str) -> Result<Goal, AppError> {
        Ok(db
            .table("goals")
            .owner(user)
            .select()?
            .id(id)
            .query_one::<Goal>()
            .await?)
    }

    pub async fn put(
        db: Arc<Client<'_>>,
        user: &str,
        goals: Vec<Goal>,
    ) -> Result<Vec<Goal>, AppError> {
        Ok(db.table("goals").owner(user).update(goals).await?)
    }

    pub async fn delete(
        db: Arc<Client<'_>>,
        user: &str,
        goals: Vec<Goal>,
    ) -> Result<Vec<Goal>, AppError> {
        Ok(db.table("goals").owner(user).delete(goals).await?)
    }

    pub async fn create(
        db: Arc<Client<'_>>,
        user: &str,
        goals: Vec<Goal>,
    ) -> Result<Vec<Goal>, AppError> {
        Ok(db.table("goals").owner(user).create(goals).await?)
    }
}
