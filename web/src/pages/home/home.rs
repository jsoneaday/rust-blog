use leptos::logging::log;
use leptos::*;
use crate::common::components::post::post_preview::PostPreviewParams;
use crate::common::components::{layout::Layout, post::post_preview::PostPreview};
use crate::common::api::api_service::ApiService;

#[component]
pub fn Home() -> impl IntoView {
    let (last_offset, _set_last_offset) = create_signal(0);
    let api_service = expect_context::<ReadSignal<ApiService>>();

    let posts = create_resource(last_offset, move |offset| async move {
        let result = api_service.get_untracked().get_latest_posts(offset).await;
        match result {
            Ok(data) => {
                data
            },
            Err(e) => {
                log!("Failed to get post data: {}", e);
                vec![]
            }
        }
    });

    view! {
        <Layout>
            <div class="home-menu">
                <h1>"Rust Indie Dev"</h1>
                <strong>"David Choi"</strong>
                <small style="margin-top: 1em">"This app is built entirely with Rust: Leptos, Actix Web"</small>
                <b style="margin-top: 0.5em"><i><small>"contact me"</small></i></b>
            </div>
            <div class="home-content">
                <ul>
                    <For
                        each=move || match posts.get() {
                            None => vec![],
                            Some(data) => data
                        }
                        key=|post| post.id
                        children=move |post| {
                            view! {
                                <PostPreview post=PostPreviewParams {
                                    id: post.id,
                                    title: post.title.to_string(),
                                    content: post.message.to_string()
                                } />
                            }
                        }
                    />
                </ul>
            </div>
        </Layout>
    }
}