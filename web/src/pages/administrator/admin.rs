use leptos::*;
use leptos::logging::log;
use leptos_router::*;
use crate::common::components::layout::Layout;

const MAIL: &str = "/admin/mail";
const ADD_POST: &str = "/admin/addpost";
const MNG_POST: &str = "/admin/mngpost";

#[component]
pub fn Admin() -> impl IntoView {
    let location = use_location();
    log!("pathname: {}", location.pathname.get());
    let (current_selected_nav, set_current_selected_nav) = create_signal(location.pathname.get());
    
    view! {
        <Layout>            
            <nav class="home-menu">
                <h2>Administration</h2>
                <ul>
                    <li>
                        <a href={MAIL} class=("a-selected", move || current_selected_nav() == MAIL ) on:click=move |_| {
                            set_current_selected_nav(MAIL.to_string());
                        }>"Mail"</a>
                    </li>
                    <li>
                        <a href={ADD_POST} class=("a-selected", move || current_selected_nav() == ADD_POST ) on:click=move |_| {
                            set_current_selected_nav(ADD_POST.to_string());
                        }>"Add Post"</a>
                    </li>
                    <li>
                        <a href={MNG_POST} class=("a-selected", move || current_selected_nav() == MNG_POST ) on:click=move |_| {
                            set_current_selected_nav(MNG_POST.to_string());
                        }>"Manage Posts"</a>
                    </li>
                </ul>
            </nav>
            <Outlet />
        </Layout>
    }
}