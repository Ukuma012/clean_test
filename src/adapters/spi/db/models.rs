use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::adapters::spi::db::schema::*;

#[derive(Queryable, QueryableByName, Insertable)]
#[table_name = "dog_facts"]
pub struct DogFact {
    pub id: i32,
    pub fact: String,
}

#[derive(Queryable, Insertable)]
#[table_name = "invitations"]
pub struct Invitation {
    pub invitation_token: uuid::Uuid,
    pub email: String,
    pub used: bool,
    pub expires_at: NaiveDateTime,
}

impl<T> From<T> for Invitation
where
    T: Into<String>,
{
    fn from(email: T) -> Self {
        Invitation {
            invitation_token: Uuid::new_v4(),
            email: email.into(),
            used: false,
            expires_at: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
        }
    }
}
