use serde::{Deserialize, Serialize};
use derive_more::Display;

#[derive(Deserialize, Display, Debug)]
pub struct OutputId {
    pub id: i64
}

#[derive(Serialize, Clone)]
pub struct NewPost {
    pub title: String,
    pub message: String,
    pub admin_id: i64
}

#[derive(Serialize, Clone)]
pub struct LoginCredential {
    pub email: String,
    pub password: String
}