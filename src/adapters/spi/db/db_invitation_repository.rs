use async_trait::async_trait;
use diesel::prelude::*;
use std::error::Error;

use crate::adapters::spi::db::db_mappers::InvitationDbMapper;
use crate::adapters::spi::db::{db_connection::DbConnection, models::Invitation, schema::invitations};
use crate::application::mappers::db_mapper::DbMapper;
use crate::application::repositories::invitation_repository_abstract::InvitationRepositoryAbstract;
use crate::domain::invitation_entity::InvitationEntity;

pub struct InvitationRepository {
    pub db_connection: DbConnection,
}

#[async_trait(?Send)]
impl InvitationRepositoryAbstract for InvitationRepository {
    async fn post_invitation(&self, user_email: String) -> Result<InvitationEntity, Box<dyn Error>> {
        let conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let new_invitation: Invitation = user_email.into();

        let result = diesel::insert_into(invitations::table).values(&new_invitation).get_result::<Invitation>(&conn);

        match result {
            Ok(model) => Ok(InvitationDbMapper::to_entity(model)),
            Err(e) => Err(Box::new(e)),
        }
    }
}
