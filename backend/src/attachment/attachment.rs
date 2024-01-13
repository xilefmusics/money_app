use crate::error::AppError;

use std::fs::File;
use std::io::Write;
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

    pub fn save(attachment: &Attachment, bytes: &[u8]) -> Result<(), AppError> {
        let mut file = File::create(Self::file_name(attachment)?)?;
        file.write_all(bytes)?;
        Ok(())
    }
}
