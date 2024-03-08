use async_trait::async_trait;

use crate::error::AppError;

#[cfg(test)]
use mockall::{predicate::*, *};

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait LoginRepositoryAbstract {
    async fn retrieval_user(&self, email: String, naive_password: String) -> Result<(), AppError>;
}
