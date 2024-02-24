use crate::adapters::api::healthcheck::healthcheck_controllers;
use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
    config.service(web::scope("/api/v1/healthcheck").configure(healthcheck_controllers::routes));
}
