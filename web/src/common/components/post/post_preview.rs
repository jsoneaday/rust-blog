use leptos::*;
use serde::{Deserialize, Serialize};
use crate::common::utils::markdown_to_html::MarkdownToHtmlConverter;

#[derive(Clone, Deserialize, Serialize)]
pub struct PostPreviewParams {
    pub id: i64,
    pub title: String,
    pub content: String
}

#[component]
pub fn PostPreview(post: PostPreviewParams) -> impl IntoView {    
    let (content, _set_content) = create_signal(post.content);
    let html_content = move || {
        let md_to_html = MarkdownToHtmlConverter::new();
        md_to_html.convert_md_to_html(content())
    };

    view! {
        <section>
            <h1>{post.title}</h1>
            <div>{html_content}</div>
        </section>
    }
}