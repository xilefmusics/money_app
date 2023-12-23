use crate::error::AppError;

use actix_web::HttpRequest;

pub fn parse_user_header(req: HttpRequest) -> Result<String, AppError> {
    Ok(req
        .headers()
        .get("X-Remote-User")
        .ok_or(AppError::Unauthorized("no X-Remote-User given".into()))?
        .to_str()
        .map_err(|_| AppError::Unauthorized("no X-Remote-User given".into()))?
        .into())
}
