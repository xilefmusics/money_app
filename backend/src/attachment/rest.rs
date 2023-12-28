use super::Attachment;

use crate::error::AppError;
use crate::rest::parse_user_header;

use std::path::PathBuf;

use fancy_surreal::Client;

use actix_files::NamedFile;
use actix_web::{get, post, web::Bytes, web::Data, web::Path, HttpRequest, HttpResponse};

#[get("/api/attachments/{id}")]
pub async fn get_id(
    req: HttpRequest,
    db: Data<Client>,
    id: Path<String>,
) -> Result<NamedFile, AppError> {
    Ok(NamedFile::open(
        PathBuf::from(std::env::var("ATTACHMENT_DIR").unwrap_or("attachments".into())).join(
            PathBuf::from(
                &Attachment::get_record(db.into_inner(), &parse_user_header(&req)?, &id)
                    .await?
                    .file_name()?,
            ),
        ),
    )?)
}

#[post("/api/attachments")]
pub async fn post(
    req: HttpRequest,
    db: Data<Client>,
    payload: Bytes,
) -> Result<HttpResponse, AppError> {
    let user = parse_user_header(&req)?;
    let attachments = Attachment::create_record(
        db.into_inner(),
        &user,
        Attachment {
            id: None,
            content_type: req
                .headers()
                .get("Content-Type")
                .ok_or(AppError::Other("No content type given".into()))?
                .to_str()
                .map_err(|_| AppError::Other("No content type given".into()))?
                .into(),
        },
    )
    .await?;
    let attachment = attachments
        .get(0)
        .ok_or(AppError::Other("no attachment created".into()))?;

    attachment.save(&payload.to_vec())?;

    Ok(HttpResponse::Created().json(attachment))
}
