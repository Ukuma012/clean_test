use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;
use uuid::Uuid;

use crate::adapters::spi::db::db_mappers::InvitationDbMapper;
use crate::adapters::spi::db::{db_connection::DbConnection, models::Invitation, schema::invitations};
use crate::application::mappers::db_mapper::DbMapper;
use crate::application::repositories::invitation_repository_abstract::InvitationRepositoryAbstract;
use crate::domain::invitation_entity::InvitationEntity;
use crate::error::AppError;

use super::schema::invitations::{invitation_token, used};

pub struct InvitationRepository {
    pub db_connection: DbConnection,
}

#[async_trait(?Send)]
impl InvitationRepositoryAbstract for InvitationRepository {
    async fn insert_invitation(&self, user_email: String) -> Result<InvitationEntity, Box<dyn Error>> {
        let conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let new_invitation: Invitation = user_email.into();

        let result = diesel::insert_into(invitations::table).values(&new_invitation).get_result::<Invitation>(&conn);

        match result {
            Ok(model) => Ok(InvitationDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }

    async fn validate_invitation_token(&self, token: Uuid) -> Result<bool, AppError> {
        let conn = self.db_connection.get_pool().get()?;

        let searched_result: Invitation = invitations::table
            .filter(invitation_token.eq(token))
            .filter(used.eq(false))
            .first::<Invitation>(&conn)
            .map_err(|e| <diesel::result::Error as Into<AppError>>::into(e))?;

        if searched_result.expires_at > chrono::Local::now().naive_local() {
            Ok(true)
        } else {
            Err(AppError::Unauthorized("Invalid invitation token".into()))
        }
    }
}
