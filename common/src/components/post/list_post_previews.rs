use leptos::*;
use crate::{api::models::Post, components::post::post_preview::{PostPreview, PostPreviewParams}, utils::date_time::convert_datetime_long_readable};

#[component]
pub fn ListPostPreviews(posts: Resource<i32, Vec<Post>>, editable: bool) -> impl IntoView {
    view! {
        <div class="home-content">
            <ul>
                <For
                    each=move || match posts() {
                        None => vec![],
                        Some(data) => data
                    }
                    key=|post| post.id
                    children=move |post| {
                        view! {
                            <div style="margin-bottom: 6em">
                                <div></div>
                                <PostPreview post=PostPreviewParams {
                                    id: post.id,
                                    updated_at: convert_datetime_long_readable(post.updated_at),
                                    title: post.title.to_string(),
                                    content: post.message.to_string(),
                                    editable
                                } />
                            </div>
                        }
                    }
                />
            </ul>
        </div>
    }
}