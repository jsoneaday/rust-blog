use serde::{Deserialize, Serialize};
use derive_more::Display;
use chrono::{DateTime, Utc};

#[derive(Deserialize, Display, Debug)]
pub struct OutputId {
    pub id: i64
}

/// Used as parameter type to create new Post
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct LoginResponse {
    pub access_token: String,
    pub login_user_id: i64
}

/// Receiving type for post queries
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Post {
    pub id: i64,
    pub updated_at: DateTime<Utc>,
    pub title: String,
    pub message: String,
    pub admin_id: i64
}