use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserEntity {
    pub email: String,
    pub password: String,
}

impl UserEntity {
    pub fn new(email: String, password: String) -> Self {
        UserEntity { email, password }
    }
}
