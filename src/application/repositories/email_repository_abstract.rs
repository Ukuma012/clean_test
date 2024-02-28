use std::error::Error;

use async_trait::async_trait;

#[async_trait(?Send)]
pub trait EmailRepositoryAbstract {
    fn send_email(&self, recipient: &str, subject: &str, body: String) -> Result<(), Box<dyn Error>>;
}
