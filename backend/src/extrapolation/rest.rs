use super::create_extrapolation;
use super::QueryParams;

use crate::error::AppError;
use crate::rest::parse_user_header;

use chrono::{Datelike, Local};
use fancy_surreal::Client;

use actix_web::{get, web::Data, web::Query, HttpRequest, HttpResponse};

#[get("/api/extrapolation")]
pub async fn get(
    req: HttpRequest,
    db: Data<Client>,
    q: Query<QueryParams>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        create_extrapolation(
            db.into_inner(),
            &parse_user_header(&req)?,
            q.year.unwrap_or(Local::now().year()),
        )
        .await?,
    ))
}
