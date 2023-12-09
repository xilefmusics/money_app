use super::Select;

use crate::error::AppError;
use crate::settings::Settings;

use serde::de::DeserializeOwned;
use serde::Serialize;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

#[derive(Debug, Clone)]
pub struct Database {
    client: Surreal<Client>,
}

impl Database {
    pub fn select(&self) -> Select {
        Select::new(&self.client)
    }

    pub async fn new(settings: Settings) -> Self {
        let client = Surreal::new::<Ws>(format!("{}:{}", settings.db_host, settings.db_port))
            .await
            .unwrap();
        client
            .signin(Root {
                username: &settings.db_user,
                password: &settings.db_password,
            })
            .await
            .unwrap();
        client
            .use_ns(settings.db_namespace)
            .use_db(settings.db_database)
            .await
            .unwrap();
        Self { client }
    }

    pub async fn create<I: Serialize, O: DeserializeOwned>(
        &self,
        table: &str,
        content: Vec<I>,
    ) -> Result<Vec<O>, AppError> {
        self.client
            .create(table)
            .content(content.get(0))
            .await
            .map_err(|err| AppError::Database(format!("{}", err)))
    }
}
