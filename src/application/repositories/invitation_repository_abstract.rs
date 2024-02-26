use async_trait::async_trait;

use crate::domain::invitation_entity::InvitationEntity;
use std::error::Error;

#[async_trait(?Send)]
pub trait InvitationRepositoryAbstract {
    async fn post_invitation(&self, email: String) -> Result<InvitationEntity, Box<dyn Error>>;
}
