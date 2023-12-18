use super::{model, Transaction};

use crate::database::Database;
use crate::error::AppError;
use crate::rest::parse_user_header;

use actix_web::{delete, get, post, web::Data, web::Json, web::Path, HttpRequest, HttpResponse};

#[get("/api/transactions")]
pub async fn get(req: HttpRequest, db: Data<Database>) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(model::select(db.select().user(&parse_user_header(req)?)).await?))
}

#[get("/api/transactions/{id:transaction.*}")]
pub async fn get_id(
    req: HttpRequest,
    db: Data<Database>,
    id: Path<String>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().json(
        model::select(
            db.select()
                .user(&parse_user_header(req)?)
                .id(&id.into_inner()),
        )
        .await?,
    ))
}

#[delete("/api/transactions")]
pub async fn delete(
    transactions: Json<Vec<Transaction>>,
    db: Data<Database>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::NoContent().json(model::delete(&db, transactions.into_inner()).await?))
}

#[post("/api/transactions")]
pub async fn post(
    req: HttpRequest,
    transactions: Json<Vec<Transaction>>,
    db: Data<Database>,
) -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok()
        .json(model::create(&db, parse_user_header(req)?, transactions.into_inner()).await?))
}
