use leptos::*;
use leptos_meta::*;
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
        let mut html = md_to_html.convert_md_to_html(content());
        let inner_text = html.clone().last().unwrap().inner_text();
        // previews are cut short, end with ellipsis
        html.last_mut().unwrap().set_inner_text(format!("{}{}", inner_text, " ...").as_str());

        html
    };
    let meta_content = move || {
        if content().len() == 0 {
            "".to_string()
        } else {
            let short_content = if content().len() < 100 {
                content()
            } else {
                (&content()[0..100]).to_string()
            };
            short_content.to_string()
        }
     };

    view! {
        <section>
            <Meta name="description" content=meta_content />
            <h1>{post.title}</h1>
            <div class="preview-content">{html_content}</div>
        </section>
    }
}