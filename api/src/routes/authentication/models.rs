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

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub login_user_id: i64
}
