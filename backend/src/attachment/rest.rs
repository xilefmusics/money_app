use super::{Attachment, AttachmentModel};

use crate::error::AppError;
use crate::rest::parse_user_header;

use fancy_surreal::Client;

use actix_files::NamedFile;
use actix_web::{get, post, web::Bytes, web::Data, web::Path, HttpRequest, HttpResponse};

#[get("/api/attachments/{id_or_path}")]
pub async fn get_id(
    req: HttpRequest,
    db: Data<Client>,
    id_or_path: Path<String>,
) -> Result<NamedFile, AppError> {
    Ok(NamedFile::open(
        AttachmentModel::get_path_db(
            db.into_inner(),
            &parse_user_header(&req)?,
            &id_or_path.into_inner(),
        )
        .await?,
    )?)
}

#[post("/api/attachments")]
pub async fn post(
    req: HttpRequest,
    db: Data<Client>,
    payload: Bytes,
) -> Result<HttpResponse, AppError> {
    let user = parse_user_header(&req)?;
    let attachments = AttachmentModel::create_record(
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

    AttachmentModel::save(&attachment, &payload.to_vec())?;

    Ok(HttpResponse::Created().json(attachment))
}
