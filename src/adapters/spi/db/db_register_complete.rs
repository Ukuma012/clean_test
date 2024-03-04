use async_trait::async_trait;
use diesel::prelude::*;

use crate::application::mappers::db_mapper::DbMapper;
use crate::application::repositories::register_complete_repository_abstract::RegisterCompleteRepositoryAbstract;

use crate::adapters::spi::db::db_connection::DbConnection;
use crate::adapters::spi::db::db_mappers::RegisterCompleteDbMapper;
use crate::adapters::spi::db::{models::User, schema::users};
use crate::adapters::spi::shared::hasher::compute_password_hash;
use crate::domain::user_entity::UserEntity;
use crate::error::AppError;

pub struct RegisterCompleteRepository {
    pub db_connection: DbConnection,
}

#[async_trait(?Send)]
impl RegisterCompleteRepositoryAbstract for RegisterCompleteRepository {
    async fn insert_user(&self, email: String, naive_password: String) -> Result<UserEntity, AppError> {
        let conn = self.db_connection.get_pool().get().expect("couldn't get db connection from pool");

        let hashed_password = compute_password_hash(naive_password)?;

        let new_user: UserEntity = UserEntity::new(email, hashed_password);

        let db_new_user = RegisterCompleteDbMapper::to_db(new_user);

        let result = diesel::insert_into(users::table).values(&db_new_user).get_result::<User>(&conn)?;

        Ok(RegisterCompleteDbMapper::to_entity(result))
    }
}
