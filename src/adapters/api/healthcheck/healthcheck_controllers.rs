use actix_web::{get, web, HttpResponse, Responder};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(healthcheck);
}

#[get("")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("Ok\n")
}
