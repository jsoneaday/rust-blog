use leptos::*;
use leptos::logging::log;
use serde::{Deserialize, Serialize};
use crate::common::{api::models::Post, utils::date_time::convert_datetime_readable};


#[component]
pub fn PostDetail(post: Resource<i64, Option<Post>>) -> impl IntoView {    
    let updated_at = move || {
        log!("PostDetail post: {:?}", post.get());
        convert_datetime_readable(post.get().unwrap().unwrap().updated_at)
    };

    view! {
        <Suspense fallback={move || view! { <p>"Loading ..."</p> }}>
            <div class="post-detail-container">
                {move || post().map(|p| view! { 
                    <div>{p.clone().unwrap().title}</div>
                    <div>{p.unwrap().message}</div>
                })}
            </div>
        </Suspense>
    }
}