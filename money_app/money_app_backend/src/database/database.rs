use super::id::{string2record, IdGetter};
use super::Select;

use crate::error::AppError;
use crate::settings::Settings;

use futures::future::join_all;
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

    pub fn select(&self) -> Select {
        Select::new(&self.client)
    }

    pub async fn query<T: Serialize + DeserializeOwned + Clone + std::fmt::Debug>(
        &self,
        query: &str,
    ) -> Result<Vec<T>, AppError> {
        self.client
            .query(query)
            .await
            .map_err(|err| AppError::Database(format!("{}", err)))?
            .take(0)
            .map_err(|err| AppError::Database(format!("{}", err)))
    }

    async fn create_one<I: Serialize, O: DeserializeOwned>(
        &self,
        table: &str,
        content: I,
    ) -> Result<Vec<O>, AppError> {
        self.client
            .create(table)
            .content(content)
            .await
            .map_err(|err| AppError::Database(format!("{}", err)))
    }

    pub async fn create<I: Serialize, O: DeserializeOwned>(
        &self,
        table: &str,
        content: Vec<I>,
    ) -> Result<Vec<O>, AppError> {
        join_all(
            content
                .into_iter()
                .map(|content| self.create_one(table, content)),
        )
        .await
        .into_iter()
        .try_fold(Vec::new(), |acc, result| {
            result.and_then(|inner_vec| {
                let mut acc = acc;
                acc.extend(inner_vec);
                Ok(acc)
            })
        })
    }

    async fn put_one<I: Serialize + IdGetter, O: DeserializeOwned>(
        &self,
        content: I,
    ) -> Result<Vec<O>, AppError> {
        self.client
            .create((content.get_id_first(), content.get_id_second()))
            .content(content)
            .await
            .map_err(|err| AppError::Database(format!("{}", err)))?
            .ok_or(AppError::Database("record is none".into()))
    }

    async fn delete_one<O: DeserializeOwned>(&self, id: String) -> Result<Option<O>, AppError> {
        let record = string2record(&id)?;
        self.client
            .delete((record.tb, record.id.to_string()))
            .await
            .map_err(|err| AppError::Database(format!("{}", err)))
    }

    pub async fn delete<O: DeserializeOwned>(
        &self,
        ids: Vec<String>,
    ) -> Result<Vec<Option<O>>, AppError> {
        join_all(ids.into_iter().map(|id| self.delete_one(id)))
            .await
            .into_iter()
            .try_fold(Vec::new(), |acc, result| {
                result.and_then(|inner_vec| {
                    let mut acc = acc;
                    acc.extend(inner_vec);
                    Ok(acc)
                })
            })
    }
}
