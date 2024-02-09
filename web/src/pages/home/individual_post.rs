use leptos::logging::log;
use leptos::*;
use leptos_router::*;
use leptos_meta::Title;
use crate::common::components::post::post_detail::PostDetail;
use crate::common::api::api_service::ApiService;
use crate::common::components::layout::Layout;

#[derive(Params, PartialEq)]
struct GetPostParams {
    post_id: i64
}

#[component]
pub fn IndividualPost() -> impl IntoView {
    let post_params = use_params::<GetPostParams>();
    let post_id = move || {
        post_params.with(|params| {
            params
            .as_ref()
            .map(|param| param.post_id)
            .unwrap_or_default()
        })
    };
    let api_service = expect_context::<ReadSignal<ApiService>>();
    let post_resource = create_resource(post_id, move |id| async move {
        log!("call post with id: {}", id);
        let result = api_service.get_untracked().get_post(id).await;
        match result {
            Ok(post) => post,
            Err(_) => panic!("A post with id {} does not exist", id)
        }
    });

    view! {
        <Layout single_column=true>
            <div class="home-content">
                <Title text=move || match post_resource() {
                    Some(p) => format!("- {}", p.unwrap().title),
                    None => "- Post".to_string()
                } />
                <PostDetail post=post_resource />
            </div>
        </Layout>
    }
}