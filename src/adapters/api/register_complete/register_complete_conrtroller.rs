use actix_session::Session;
use actix_web::{post, web, HttpResponse};

use crate::adapters::api::register_complete::register_complete_payloads::RegisterCompleteUser;
use crate::adapters::api::shared::app_state::AppState;
use crate::application::usecases::interfaces::AbstractRegisterCompleteUseCase;
use crate::application::usecases::register_complete_usecase::RegisterCompleteUseCase;
use crate::error_response::ApiResponse;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register_complete);
}

#[post("")]
async fn register_complete(session: Session, data: web::Data<AppState>, invitation_token: web::Path<String>, form: web::Json<RegisterCompleteUser>) -> ApiResponse {
    let register_complete_usecase = RegisterCompleteUseCase::new(&form.email, &form.password, &invitation_token, &data.invitation_repository, &data.register_complete_repository);
    let user = register_complete_usecase.register().await?;
    Ok(HttpResponse::Ok().json(user))
}
