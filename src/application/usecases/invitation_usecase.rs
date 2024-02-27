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
        let invitation = self.invitation_repository.post_invitation(self.email.to_string()).await;

        let recipient = "test@hardcode.com";
        let subject = "hardcode subject";
        let body = "This is hardcoded text";

        let _send_email = self.email_repository.send_email(recipient, subject, body);

        match invitation {
            Ok(invitation) => Ok(invitation),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot insert invitation", Some(e))),
        }
    }
}
