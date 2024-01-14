use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct RefreshToken {
    pub old_token: String
}

#[derive(Deserialize)]
pub struct LoginCredential {
    pub email: String,
    pub password: String
}
