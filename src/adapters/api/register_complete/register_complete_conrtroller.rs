use actix_session::Session;
use actix_web::{post, web, HttpResponse};

use crate::adapters::api::register_complete::register_complete_payloads::RegisterCompleteUser;
use crate::adapters::api::shared::app_state::AppState;
use crate::adapters::api::shared::session::{self, generate_session_id, set_session_id};
use crate::application::usecases::interfaces::AbstractRegisterCompleteUseCase;
use crate::application::usecases::register_complete_usecase::RegisterCompleteUseCase;
use crate::error_response::ApiResponse;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(register_complete);
}

#[post("")]
async fn register_complete(data: web::Data<AppState>, invitation_token: web::Path<String>, form: web::Json<RegisterCompleteUser>, session: Session) -> ApiResponse {
    let register_complete_usecase = RegisterCompleteUseCase::new(&form.email, &form.password, &invitation_token, &data.invitation_repository, &data.register_complete_repository);
    let user = register_complete_usecase.register().await?;

    let new_session_id = generate_session_id()?;
    set_session_id(&session, &new_session_id)?;
    println!("{}", new_session_id);
    Ok(HttpResponse::Ok().json(user))
}
