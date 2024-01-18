use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct PostPreviewParams {
    pub id: i64,
    pub title: String,
    pub content: String
}

#[component]
pub fn PostPreview(post: PostPreviewParams) -> impl IntoView {
    view! {
        <div>{post.title}</div>
        <div>{post.content}</div>
    }
}