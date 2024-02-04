use leptos::*;
use leptos::logging::log;
use leptos_router::*;
use crate::common::api::models::LoginResponse;
use crate::common::components::layout::Layout;
use crate::common::components::authentication::login::Login;
use crate::common::components::modal::Modal;

const MAIL: &str = "/admin/mail";
const ADD_POST: &str = "/admin/add_edit";
const MNG_POST: &str = "/admin/mngpost";

#[component]
pub fn Admin() -> impl IntoView {
    let location = use_location();
    log!("pathname: {}", location.pathname.get_untracked());
    let (current_selected_nav, set_current_selected_nav) = create_signal(location.pathname.get_untracked());
    let (dialog_open, set_dialog_open) = create_signal(false);    
    let (login_resp, _) = expect_context::<(ReadSignal<Option<LoginResponse>>, WriteSignal<Option<LoginResponse>>)>();

    create_effect(move |_| {
        if let None = login_resp() {
            log!("login_resp changed: {}", true);
            set_dialog_open(true);
        } else {
            log!("login_resp changed: {}", false);
            set_dialog_open(false);
        }
    });
    
    view! {
        <Layout single_column=false>            
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
                        }>"Add Edit Post"</a>
                    </li>
                    <li>
                        <a href={MNG_POST} class=("a-selected", move || current_selected_nav() == MNG_POST ) on:click=move |_| {
                            set_current_selected_nav(MNG_POST.to_string());
                        }>"Manage Posts"</a>
                    </li>
                    <li>
                        <Modal disable_dismiss=true open_state=dialog_open set_open_state=set_dialog_open>
                            <Login />
                        </Modal>
                    </li>
                </ul>
            </nav>
            <Outlet />
        </Layout>
    }
}