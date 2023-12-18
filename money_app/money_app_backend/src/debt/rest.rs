use super::model;

use crate::database::Database;
use crate::error::AppError;

use actix_web::{get, web::Data, HttpRequest, HttpResponse};

#[get("/api/debt")]
pub async fn get(req: HttpRequest, db: Data<Database>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(model::select(&db, "xilef").await?))
}
