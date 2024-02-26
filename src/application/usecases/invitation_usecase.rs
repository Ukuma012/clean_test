use async_trait::async_trait;

use crate::application::{
    repositories::invitation_repository_abstract::InvitationRepositoryAbstract, usecases::interfaces::AbstractUseCase, utils::error_handling_utils::ErrorHandlingUtils,
};
use crate::domain::{error::ApiError, invitation_entity::InvitationEntity};

pub struct InvitationUseCase<'a> {
    email: &'a String,
    repository: &'a dyn InvitationRepositoryAbstract,
}

impl<'a> InvitationUseCase<'a> {
    pub fn new(email: &'a String, repository: &'a dyn InvitationRepositoryAbstract) -> Self {
        InvitationUseCase { email, repository }
    }
}

#[async_trait(?Send)]
impl<'a> AbstractUseCase<InvitationEntity> for InvitationUseCase<'a> {
    async fn execute(&self) -> Result<InvitationEntity, ApiError> {
        let invitation = self.repository.post_invitation(self.email.to_string()).await;

        match invitation {
            Ok(invitation) => Ok(invitation),
            Err(e) => Err(ErrorHandlingUtils::application_error("Cannot insert invitation", Some(e))),
        }
    }
}
