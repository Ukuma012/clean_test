use async_trait::async_trait;

use crate::domain::invitation_entity::InvitationEntity;

#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait InvitationRepositoryAbstract {
    async fn insert_invitation(&self, email: String) -> Result<InvitationEntity, Box<dyn Error>>;
}
