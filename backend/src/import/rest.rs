use super::import as import_fn;

use crate::error::AppError;
use crate::rest::parse_user_header;
use crate::transaction::TransactionModel;

use fancy_surreal::Client;

use actix_web::{post, web::Data, HttpRequest, HttpResponse};

#[post("/api/import")]
pub async fn import(
    req: HttpRequest,
    db: Data<Client>,
    payload: String,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Created().json(
        TransactionModel::create(
            db.into_inner(),
            &parse_user_header(&req)?,
            import_fn(&payload)?,
        )
        .await?,
    ))
}
