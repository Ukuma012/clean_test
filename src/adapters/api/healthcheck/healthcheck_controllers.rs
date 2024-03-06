use crate::{adapters::api::shared::session::validate_session, error::AppError};
use actix_session::Session;
use actix_web::{get, web, HttpResponse};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(healthcheck);
}

#[get("")]
async fn healthcheck(session: Session) -> Result<HttpResponse, AppError> {
    validate_session(&session)?;
    Ok(HttpResponse::Ok().body("Ok\n"))
}
