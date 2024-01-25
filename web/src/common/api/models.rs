use serde::Serialize;

#[derive(Serialize, Clone)]
pub struct NewPost {
    pub message: String,
    pub admin_id: i64
}