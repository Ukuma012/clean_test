use crate::adapters::api::{healthcheck::healthcheck_controllers, invitation::invitation_controller, login::login_controller, register_complete::register_complete_conrtroller};
use actix_web::web;

pub fn routes(config: &mut web::ServiceConfig) {
    config
        .service(web::scope("/api/v1/healthcheck").configure(healthcheck_controllers::routes))
        .service(web::scope("/api/v1/register/invitation").configure(invitation_controller::routes))
        .service(web::scope("/api/v1/register/complete/{invitation_token}").configure(register_complete_conrtroller::routes))
        .service(web::scope("api/v1/login").configure(login_controller::routes));
}
