use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;


#[derive(PartialEq)]
pub enum AuthenticateResult {
    Success{ id: i64 },
    Failure
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub user_name: String,
    pub email: String,
    pub password: String
}