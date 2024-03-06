use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEntity {
    pub email: String,
    pub password: String,
    pub session_id: Option<uuid::Uuid>,
}

impl UserEntity {
    pub fn new(email: String, password: String) -> Self {
        UserEntity { email, password, session_id: None }
    }
}
