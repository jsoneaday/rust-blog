use leptos::logging::log;
use leptos::*;
use leptos_router::A;
use leptos_meta::*;
use rustyindie_common::components::post::post_preview::PostPreviewParams;
use rustyindie_common::components::{layout::Layout, post::post_preview::PostPreview, post::list_post_previews::ListPostPreviews};
use rustyindie_common::api::api_service::ApiService;
use rustyindie_common::utils::date_time::convert_datetime_long_readable;


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
        <Layout single_column=false>
            <Title text="- Blog Posts" />
            <div class="home-menu">
                <h1>"Rust Indie Dev"</h1>
                <h2 style="margin-top: -0.4em">"Indie Development with Rust"</h2>
                <strong>"David Choi"</strong>
                <a href="mailto:admin@dzhaven.com"><b style="margin-top: 0.5em"><i><small>"contact me"</small></i></b></a>
                <div style="margin-top: 1em">
                    <A href="https://github.com/jsoneaday/rust-blog" target="_blank">
                        <small>"This app is built entirely with Rust: Leptos, Actix Web"</small>
                    </A>
                </div>                
            </div>
            <ListPostPreviews posts=posts editable=false />
        </Layout>
    }
}