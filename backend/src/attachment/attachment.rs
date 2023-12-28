use crate::error::AppError;

use std::fs::File;
use std::io::Write;
use std::sync::Arc;

use fancy_surreal::{Client, Databasable};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attachment {
    pub id: Option<String>,
    pub content_type: String,
}

impl Databasable for Attachment {
    fn get_id(&self) -> Option<String> {
        self.id.clone()
    }

    fn set_id(&mut self, id: Option<String>) {
        self.id = id;
    }
}

impl Attachment {
    pub async fn create_record(
        db: Arc<Client>,
        user: &str,
        attachment: Self,
    ) -> Result<Vec<Self>, AppError> {
        Ok(db
            .table("attachments")
            .owner(user)
            .create_one(attachment)
            .await?)
    }

    pub async fn get_record(db: Arc<Client>, user: &str, id: &str) -> Result<Self, AppError> {
        Ok(db
            .table("attachments")
            .owner(user)
            .select()?
            .id(id)
            .query_one::<Self>()
            .await?)
    }

    pub fn extension(&self) -> Result<&'static str, AppError> {
        match self.content_type.as_ref() {
            "application/pdf" => Ok("pdf"),
            "image/jpeg" => Ok("jpeg"),
            "image/jpg" => Ok("jpg"),
            "image/png" => Ok("png"),
            _ => Err(AppError::Other(format!(
                "unknown content-type: {}",
                self.content_type
            ))),
        }
    }

    pub fn file_name(&self) -> Result<String, AppError> {
        Ok(format!(
            "{}.{}",
            self.id.clone().ok_or(AppError::Other("no id".into()))?,
            self.extension()?
        ))
    }

    pub fn save(&self, bytes: &[u8]) -> Result<(), AppError> {
        let mut file = File::create(&self.file_name()?)?;
        file.write_all(bytes)?;
        Ok(())
    }
}
