use super::{Contract, ContractModel};

use crate::error::AppError;
use crate::rest::parse_user_header;

use fancy_surreal::Client;

use actix_web::{
    delete, get, post, put, web::Data, web::Json, web::Path, HttpRequest, HttpResponse,
};

#[get("/api/contracts")]
pub async fn get(req: HttpRequest, db: Data<Client>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(ContractModel::get(db.into_inner(), &parse_user_header(&req)?).await?))
}

#[get("/api/contracts/{id}")]
pub async fn get_id(
    req: HttpRequest,
    db: Data<Client>,
    id: Path<String>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        ContractModel::get_one(db.into_inner(), &parse_user_header(&req)?, &id.into_inner()).await?,
    ))
}

#[put("/api/contracts")]
pub async fn put(
    req: HttpRequest,
    contracts: Json<Vec<Contract>>,
    db: Data<Client>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Created().json(
        ContractModel::put(
            db.into_inner(),
            &parse_user_header(&req)?,
            contracts.into_inner(),
        )
        .await?,
    ))
}

#[delete("/api/contracts")]
pub async fn delete(
    req: HttpRequest,
    contracts: Json<Vec<Contract>>,
    db: Data<Client>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::NoContent().json(
        ContractModel::delete(
            db.into_inner(),
            &parse_user_header(&req)?,
            contracts.into_inner(),
        )
        .await?,
    ))
}

#[post("/api/contracts")]
pub async fn post(
    req: HttpRequest,
    contracts: Json<Vec<Contract>>,
    db: Data<Client>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Created().json(
        ContractModel::create(
            db.into_inner(),
            &parse_user_header(&req)?,
            contracts.into_inner(),
        )
        .await?,
    ))
}
