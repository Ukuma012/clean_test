use actix_session::Session;
use actix_web::{post, web, HttpResponse};

use crate::adapters::api::login::login_payloads::LoginUser;
use crate::adapters::api::shared::session;
use crate::application::usecases::interfaces::AbstractLoginUseCase;
use crate::{adapters::api::shared::app_state::AppState, application::usecases::login_usecase::LoginUseCase, error_response::ApiResponse};
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}

#[post("")]
async fn login(data: web::Data<AppState>, form: web::Json<LoginUser>, session: Session) -> ApiResponse {
    let login_usecase = LoginUseCase::new(&form.email, &form.password, &data.login_repository);
    login_usecase.login().await?;
    Ok(HttpResponse::Ok().finish())
}
