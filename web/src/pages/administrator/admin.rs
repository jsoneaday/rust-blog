use leptos::*;
use leptos_router::*;
use crate::common::components::layout::Layout;

#[component]
pub fn Admin() -> impl IntoView {
    view! {
        <Layout>
            <nav class="home-menu">
                <ul>
                    <li>
                        <A href="mail">"Mail"</A>
                    </li>
                    <li>
                        <A href="addpost">"Add Post"</A>
                    </li>
                    <li>
                        <A href="mngpost">"Manage Posts"</A>
                    </li>
                </ul>
            </nav>
            <Outlet />
        </Layout>
    }
}