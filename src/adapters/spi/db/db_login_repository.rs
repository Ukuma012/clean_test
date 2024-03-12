use async_trait::async_trait;
use diesel::prelude::*;

use crate::application::repositories::login_repository_abstract::LoginRepositoryAbstract;

use crate::adapters::spi::db::db_connection::DbConnection;
use crate::adapters::spi::db::{models::User, schema::users::dsl::*};
use crate::adapters::spi::shared::hasher::verify_password_hash;
use crate::error::AppError;

pub struct LoginRepository {
    pub db_connection: DbConnection,
}

#[async_trait(?Send)]
impl LoginRepositoryAbstract for LoginRepository {
    async fn retrieval_user(&self, user_email: String, naive_password: String) -> Result<(), AppError> {
        let conn = self.db_connection.get_pool().get()?;
        let user: User = users.filter(email.eq(user_email)).get_result::<User>(&conn)?;
        let _password_verify = verify_password_hash(naive_password, user.password)?;

        Ok(())
    }
}
