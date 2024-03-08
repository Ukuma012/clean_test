use async_trait::async_trait;

use crate::{domain::user_entity::UserEntity, error::AppError};

#[cfg(test)]
use mockall::{predicate::*, *};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait RegisterCompleteRepositoryAbstract {
    async fn insert_user(&self, email: String, naive_password: String) -> Result<UserEntity, AppError>;
}
