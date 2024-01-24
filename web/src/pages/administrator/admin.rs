use leptos::*;
use leptos_router::*;
use crate::common::components::layout::Layout;

#[component]
pub fn Admin() -> impl IntoView {
    let location = use_location();
    let (current_selected_nav, set_current_selected_nav) = create_signal(location.pathname.get());
    
    view! {
        <Layout>            
            <nav class="home-menu">
                <h2>Administration</h2>
                <ul>
                    <li>
                        <a href="/admin/mail" class=("a-selected", move || current_selected_nav() == "mail" )>"Mail"</a>
                    </li>
                    <li>
                        <a href="/admin/addpost" class=("a-selected", move || current_selected_nav() == "addpost" )>"Add Post"</a>
                    </li>
                    <li>
                        <a href="/admin/mngpost" class=("a-selected", move || current_selected_nav() == "mngpost" )>"Manage Posts"</a>
                    </li>
                </ul>
            </nav>
            <Outlet />
        </Layout>
    }
}