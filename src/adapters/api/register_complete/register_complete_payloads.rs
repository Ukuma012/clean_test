use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RegisterCompleteUser {
    pub email: String,
    pub password: String,
}
