use leptos::*;
use leptos::logging::log;
use rustyindie_common::{api::models::Post, utils::date_time::convert_datetime_short_readable};
use rustyindie_common::utils::markdown_to_html::MarkdownToHtmlConverter;

#[component]
pub fn PostDetail(post: Resource<i64, Option<Post>>) -> impl IntoView {
    let html_content = move || {
        let md_to_html = MarkdownToHtmlConverter::new();
        let msg_content = post().map(|p| {
            p.unwrap().message
        }).unwrap_or_default();

        let html = md_to_html.convert_md_to_html(msg_content);
        log!("html: {:?}", html.iter().map(|h| h.outer_html()).collect::<String>());
        html
    };
    let updated_at = move || {        
        convert_datetime_short_readable(post.get().unwrap().unwrap().updated_at)
    };

    view! {
        <Suspense fallback={move || view! { <p>"Loading ..."</p> }}>
            <div class="post-detail-container">
                {move || post().map(|p| view! {
                    <div>                        
                        <h1>{p.clone().unwrap().title}</h1>
                        <small><b>{updated_at}</b></small>
                    </div>
                    <div>{html_content}</div>
                })}
            </div>
        </Suspense>
    }
}