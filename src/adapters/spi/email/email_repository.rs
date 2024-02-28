use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use std::env;
use std::error::Error;

use crate::application::repositories::email_repository_abstract::EmailRepositoryAbstract;

pub struct MailTrapRepository {}

impl EmailRepositoryAbstract for MailTrapRepository {
    fn send_email(&self, recipient: &str, subject: &str, body: String) -> Result<(), Box<dyn Error>> {
        let email = Message::builder()
            .from("Keyaki Capital <keyaki@test.com>".parse().unwrap())
            .to(recipient.parse().unwrap())
            .subject(subject)
            .body(String::from(body))
            .unwrap();

        let username: String = env::var("MAILTRAP_USERNAME").expect("MAILTRAP_USERNAME must be set");
        let password: String = env::var("MAILTRAP_PASSWORD").expect("MAILTRAP_PASSWORD must be set");

        let creds = Credentials::new(username, password);

        let mailer = SmtpTransport::starttls_relay("sandbox.smtp.mailtrap.io").unwrap().credentials(creds).build();

        let result = mailer.send(&email);

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(Box::new(e)),
        }
    }
}
