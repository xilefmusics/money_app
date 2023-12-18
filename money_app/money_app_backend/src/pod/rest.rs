use super::model;

use crate::database::Database;
use crate::error::AppError;
use crate::rest::parse_user_header;

use actix_web::{get, web::Data, HttpRequest, HttpResponse};

#[get("/api/pod")]
pub async fn get(req: HttpRequest, db: Data<Database>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(model::select(&db, &parse_user_header(req)?).await?))
}
