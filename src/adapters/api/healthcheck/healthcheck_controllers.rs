use actix_session::Session;
use actix_web::{get, post, web, Error, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(healthcheck).service(login).service(change).service(add).service(reset);
}

#[post("/login")]
async fn login(session: Session) -> Result<impl Responder, Error> {
    let json = match session.get::<Uuid>("session_id")? {
        Some(session_id) => json!({"session_id": &session_id}),
        None => {
            let session_id = Uuid::new_v4();
            session.set("session_id", &session_id)?;

            json!({"session_id": &session_id})
        }
    };

    Ok(HttpResponse::Ok().json(&json))
}

#[post("/change")]
async fn change(session: Session) -> Result<impl Responder, Error> {
    if session.get::<Uuid>("session_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let session_id = Uuid::new_v4();
    session.set("session_id", &session_id)?;

    Ok(HttpResponse::Ok().json(json! ({"session_id": &session_id})))
}

#[post("/add")]
async fn add(session: Session) -> Result<impl Responder, Error> {
    if session.get::<Uuid>("session_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let count = session.get::<u32>("count")?.unwrap_or(0) + 1;
    session.set("count", &count)?;

    Ok(HttpResponse::Ok().json(json!( {"count": &count})))
}

#[post("/reset")]
async fn reset(session: Session) -> Result<impl Responder, Error> {
    if session.get::<Uuid>("session_id")?.is_none() {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    session.set("count", 0)?;

    Ok(HttpResponse::Ok().json(json! ({"count": 0})))
}

#[get("")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("Ok\n")
}
