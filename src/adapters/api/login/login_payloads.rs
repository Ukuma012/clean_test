use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}
