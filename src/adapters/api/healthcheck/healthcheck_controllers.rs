use actix_session::Session;
use actix_web::{get, web, HttpResponse, Responder};

use crate::{
    adapters::api::shared::session::{generate_session_id, set_session_id, validate_session},
    error::AppError,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(healthcheck).service(session_set).service(session_get);
}

#[get("")]
async fn healthcheck(session: Session) -> Result<impl Responder, AppError> {
    validate_session(session.clone())?;
    let text: Option<String> = session.get("testing")?;
    println!("{:#?}", text);
    Ok(HttpResponse::Ok().body("Ok\n"))
}

#[get("/session_set")]
async fn session_set(session: Session) -> Result<impl Responder, AppError> {
    session.insert("user_id", "this should be printied")?;
    session.insert("testing", "can you see me?")?;
    Ok(HttpResponse::Ok().body("inserted\n"))
}

#[get("/session_get")]
async fn session_get(session: Session) -> Result<impl Responder, AppError> {
    let id: Option<String> = session.get("testing")?;
    println!("{:#?}", id);
    Ok(HttpResponse::Ok().body("Can you see the result?\n"))
}
