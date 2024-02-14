use leptos::logging::log;
use leptos::*;
use rustyindie_common::api::api_service::ApiService;
use rustyindie_common::components::post::list_post_previews::ListPostPreviews;

#[component]
pub fn ManagePosts() -> impl IntoView {
    let (last_offset, _set_last_offset) = create_signal(0);    
    let api_service = expect_context::<ReadSignal<ApiService>>();
  
    let posts = create_resource(last_offset, move |offset| async move {
        let result = api_service.get_untracked().get_latest_posts(offset).await;
        match result {
            Ok(data) => data,
            Err(e) => {
                log!("Failed to get post data: {}", e);
                vec![]
            }
        }
    });

    view! {
        <ListPostPreviews posts=posts editable=true />
    }
}