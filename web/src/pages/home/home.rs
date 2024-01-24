use leptos::*;
use crate::common::components::{layout::Layout, post::post_preview::PostPreview};
use crate::common::testing_utils::fake_data::get_fake_post_preview_data;

#[component]
pub fn Home() -> impl IntoView {
    let posts = create_resource(|| (), |_| async move {
        get_fake_post_preview_data().await
    });

    view! {
        <Layout>
            <div class="home-menu">
                <h1>"Rust Indie Dev"</h1>
                <strong>"David Choi"</strong>
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
                                <PostPreview post=post />
                            }
                        }
                    />
                </ul>
            </div>
        </Layout>
    }
}