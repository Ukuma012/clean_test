use async_trait::async_trait;

use crate::application::{
    repositories::email_repository_abstract::EmailRepositoryAbstract, repositories::invitation_repository_abstract::InvitationRepositoryAbstract,
    usecases::interfaces::AbstractInvitationUseCase, utils::error_handling_utils::ErrorHandlingUtils,
};
use crate::domain::{error::ApiError, invitation_entity::InvitationEntity};

pub struct InvitationUseCase<'a> {
    email: &'a String,
    invitation_repository: &'a dyn InvitationRepositoryAbstract,
    email_repository: &'a dyn EmailRepositoryAbstract,
}

impl<'a> InvitationUseCase<'a> {
    pub fn new(email: &'a String, invitation_repository: &'a dyn InvitationRepositoryAbstract, email_repository: &'a dyn EmailRepositoryAbstract) -> Self {
        InvitationUseCase {
            email,
            invitation_repository,
            email_repository,
        }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractInvitationUseCase<InvitationEntity> for InvitationUseCase<'a> {
    async fn post_invitation(&self) -> Result<InvitationEntity, ApiError> {
        let invitation = self.invitation_repository.insert_invitation(self.email.to_string()).await;

        match invitation {
            Ok(invitation) => {
                let recipient = &invitation.email;
                let subject = "invitation link";
                let body = format!(
                    "Please click on the link below to complete registration. <br/>
                        <a href=\"http://localhost:7878/api/users/complete/{}\">
                        http://localhost:7878/api/users/complete/</a><br/>
                        your invitation expires on <strong>{}</strong>",
                    invitation.invitation_token,
                    invitation.expires_at.format("%I:%M %p %A, %-d %B, %C%y").to_string()
                );

                let send_email = self.email_repository.send_email(recipient, subject, body);
                match send_email {
                    Ok(_) => Ok(invitation),
                    Err(e) => Err(ErrorHandlingUtils::application_error("Cannot send email", Some(e))),
                }
            }
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot insert invitation", Some(e))),
        }
    }
}
