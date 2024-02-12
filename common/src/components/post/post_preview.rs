use leptos::*;
use leptos_meta::Meta;
use leptos_router::A;
use serde::{Deserialize, Serialize};
use crate::utils::markdown_to_html::MarkdownToHtmlConverter;

#[derive(Clone, Deserialize, Serialize)]
pub struct PostPreviewParams {
    pub id: i64,
    /// friendly datetime string
    pub updated_at: String,
    pub title: String,
    pub content: String,
    pub editable: bool
}

#[component]
pub fn PostPreview(post: PostPreviewParams) -> impl IntoView {    
    let (content, _set_content) = create_signal(post.content);
    let (href, _set_href) = create_signal(
        if post.editable {
            format!("/add_edit/{}", post.id)
        } else {
            format!("/post/{}", post.id)
        }
    );
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
        <A href=href>
            <section>
                <Meta name="description" content=meta_content />
                <span>{post.updated_at}</span>
                <h1 style="margin-top: 0.4em">{post.title}</h1>
                <div class="preview-content">{html_content}</div>
            </section>
        </A>
    }
}