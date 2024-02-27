use chrono::NaiveDateTime;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct InvitationEntity {
    pub invitation_token: uuid::Uuid,
    pub email: String,
    pub used: bool,
    pub expires_at: NaiveDateTime,
}

impl InvitationEntity {
    pub fn new(invitation_token: uuid::Uuid, email: String, used: bool, expires_at: NaiveDateTime) -> Self {
        InvitationEntity {
            invitation_token,
            email,
            used,
            expires_at,
        }
    }
}
