use super::{Goal, GoalModel};

use crate::error::AppError;
use crate::rest::parse_user_header;

use fancy_surreal::Client;

use actix_web::{
    delete, get, post, put, web::Data, web::Json, web::Path, HttpRequest, HttpResponse,
};

#[get("/api/goals")]
pub async fn get(req: HttpRequest, db: Data<Client<'_>>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(GoalModel::get(db.into_inner(), &parse_user_header(&req)?).await?))
}

#[get("/api/goals/{id}")]
pub async fn get_id(
    req: HttpRequest,
    db: Data<Client<'_>>,
    id: Path<String>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        GoalModel::get_one(db.into_inner(), &parse_user_header(&req)?, &id.into_inner()).await?,
    ))
}

#[put("/api/goals")]
pub async fn put(
    req: HttpRequest,
    goals: Json<Vec<Goal>>,
    db: Data<Client<'_>>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Created().json(
        GoalModel::put(
            db.into_inner(),
            &parse_user_header(&req)?,
            goals.into_inner(),
        )
        .await?,
    ))
}

#[delete("/api/goals")]
pub async fn delete(
    req: HttpRequest,
    goals: Json<Vec<Goal>>,
    db: Data<Client<'_>>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::NoContent().json(
        GoalModel::delete(
            db.into_inner(),
            &parse_user_header(&req)?,
            goals.into_inner(),
        )
        .await?,
    ))
}

#[post("/api/goals")]
pub async fn post(
    req: HttpRequest,
    goals: Json<Vec<Goal>>,
    db: Data<Client<'_>>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Created().json(
        GoalModel::create(
            db.into_inner(),
            &parse_user_header(&req)?,
            goals.into_inner(),
        )
        .await?,
    ))
}
