use leptos::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct PostDetailParams {
    pub updated_at: String,
    pub title: String,
    pub content: String
}

#[component]
pub fn PostDetail(post: PostDetailParams) -> impl IntoView {
    view! {
        <div>{post.updated_at}</div>
        <div>{post.title}</div>
        <div>{post.content}</div>
    }
}