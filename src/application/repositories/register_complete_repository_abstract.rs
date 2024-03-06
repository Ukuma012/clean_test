use async_trait::async_trait;

use crate::{domain::user_entity::UserEntity, error::AppError};

#[cfg(test)]
use mockall::{predicate::*, *};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait RegisterCompleteRepositoryAbstract {
    async fn insert_user(&self, email: String, naive_password: String) -> Result<UserEntity, AppError>;
    async fn generate_session_id(&self, user: UserEntity) -> Result<UserEntity, AppError>;
}
