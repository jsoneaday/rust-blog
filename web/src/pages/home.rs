use leptos::*;
use crate::common::layout::Layout;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <Layout>
            <div>
                menu
            </div>
            <div>
                posts
            </div>
        </Layout>
    }
}