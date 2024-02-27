use actix_web::{post, web, HttpResponse};

use crate::adapters::api::invitation::invitation_payloads;
use crate::adapters::api::shared::app_state::AppState;
use crate::adapters::api::shared::error_presenter::ErrorResponse;
use crate::application::usecases::interfaces::AbstractInvitationUseCase;
use crate::application::usecases::invitation_usecase::InvitationUseCase;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(post_invitation);
}

#[post("/")]
async fn post_invitation(data: web::Data<AppState>, form: web::Json<invitation_payloads::Invitation>) -> Result<HttpResponse, ErrorResponse> {
    let invitation_usecase = InvitationUseCase::new(&form.email, &data.invitation_repository);
    let invitation = invitation_usecase.insert_invitation().await;

    invitation.map_err(ErrorResponse::map_io_error).map(|_inserted_invitation| HttpResponse::Ok().json("hi"))
}
