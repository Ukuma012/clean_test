use async_trait::async_trait;

#[cfg(test)]
use mockall::{predicate::*, *};
use std::error::Error;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait EmailRepositoryAbstract {
    fn send_email(&self, recipient: &str, subject: &str, body: String) -> Result<(), Box<dyn Error>>;
}
