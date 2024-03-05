use actix_session::Session;
use actix_web::{get, web, Error, HttpResponse, Responder};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(healthcheck);
}

// #[get("")]
// async fn healthcheck(_session: Session) -> Result<impl Responder, Error> {
//     Ok("Hello World from health check. Can you see the actix_session?\n")
// }

#[get("")]
async fn healthcheck() -> impl Responder {
    HttpResponse::Ok().body("Ok\n")
}
