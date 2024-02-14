use leptos::*;
use leptos_router::*;
use rustyindie_common::api::models::LoginResponse;
use rustyindie_common::components::layout::Layout;
use rustyindie_common::components::authentication::login::Login;
use rustyindie_common::components::modal::Modal;

const MAIL: &str = "/mail";
const ADD_EDIT_POST: &str = "/add_edit";
const MNG_POST: &str = "/mngpost";

#[component]
pub fn Admin() -> impl IntoView {
    let location = use_location();
    let (current_selected_nav, set_current_selected_nav) = create_signal(location.pathname.get_untracked());
    let (dialog_open, set_dialog_open) = create_signal(false);    
    let (login_resp, _) = expect_context::<(ReadSignal<Option<LoginResponse>>, WriteSignal<Option<LoginResponse>>)>();

    create_effect(move |_| {
        set_current_selected_nav(location.pathname.get());
    });

    create_effect(move |_| {
        if let Some(_login) = login_resp() {
            set_dialog_open(false);
        } else {
            set_dialog_open(true);
        }
    });
    
    view! {
        <Layout single_column=false>            
            <nav class="home-menu">
                <h2>Administration</h2>
                <ul>
                    <li>
                        <a href={MAIL} class=("a-selected", move || current_selected_nav() == MAIL )>"Mail"</a>
                    </li>
                    <li>
                        <a href={ADD_EDIT_POST} class=("a-selected", move || current_selected_nav() == ADD_EDIT_POST )>"Add Edit Post"</a>
                    </li>
                    <li>
                        <a href={MNG_POST} class=("a-selected", move || current_selected_nav() == MNG_POST )>"Manage Posts"</a>
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