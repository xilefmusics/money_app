use super::create_extrapolation;

use crate::contract::ContractModel;
use crate::error::AppError;
use crate::rest::parse_user_header;

use fancy_surreal::Client;

use actix_web::{get, web::Data, HttpRequest, HttpResponse};

#[get("/api/extrapolation")]
pub async fn get(req: HttpRequest, db: Data<Client>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(create_extrapolation(
        &ContractModel::get(db.into_inner(), &parse_user_header(&req)?).await?,
        50000,
    )))
}
