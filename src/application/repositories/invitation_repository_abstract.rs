use async_trait::async_trait;
use uuid::Uuid;

use crate::{domain::invitation_entity::InvitationEntity, error::AppError};

#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait InvitationRepositoryAbstract {
    async fn insert_invitation(&self, email: String) -> Result<InvitationEntity, Box<dyn Error>>;
    async fn validate_invitation_token(&self, token: Uuid) -> Result<bool, AppError>;
}
