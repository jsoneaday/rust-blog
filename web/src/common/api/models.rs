use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct NewPost {
    pub title: String,
    pub message: String,
    pub admin_id: i64
}