use chrono::{Utc, DateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Mail {
    pub id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub from: String,
    pub subject: String,
    pub message: String
}