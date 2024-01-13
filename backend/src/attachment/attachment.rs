use crate::error::AppError;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;

use fancy_surreal::Client;

pub use money_app_shared::attachment::Attachment;

pub struct AttachmentModel {}

impl AttachmentModel {
    pub async fn create_record(
        db: Arc<Client>,
        user: &str,
        attachment: Attachment,
    ) -> Result<Vec<Attachment>, AppError> {
        Ok(db
            .table("attachments")
            .owner(user)
            .create_one(attachment)
            .await?)
    }

    pub async fn get_record(db: Arc<Client>, user: &str, id: &str) -> Result<Attachment, AppError> {
        Ok(db
            .table("attachments")
            .owner(user)
            .select()?
            .id(id)
            .query_one::<Attachment>()
            .await?)
    }

    pub fn file_name(attachment: &Attachment) -> Result<String, AppError> {
        Ok(format!(
            "{}.{}",
            attachment
                .id
                .clone()
                .ok_or(AppError::Other("no id".into()))?,
            attachment.extension()
        ))
    }

    pub fn get_path(attachment: &Attachment) -> Result<PathBuf, AppError> {
        let dir_name =
            PathBuf::from(std::env::var("ATTACHMENT_DIR").unwrap_or("attachments".into()));
        let file_name = Self::file_name(&attachment)?;
        Ok(dir_name.join(file_name))
    }

    pub fn save(attachment: &Attachment, bytes: &[u8]) -> Result<(), AppError> {
        let mut file = File::create(Self::get_path(attachment)?)?;
        file.write_all(bytes)?;
        Ok(())
    }

    pub async fn get_path_db(
        db: Arc<Client>,
        user: &str,
        id_or_path: &str,
    ) -> Result<PathBuf, AppError> {
        let id = id_or_path.split(".").next().unwrap_or("not_an_id");
        Self::get_path(&AttachmentModel::get_record(db, user, &id).await?)
    }
}
