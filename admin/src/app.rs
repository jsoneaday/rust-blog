use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use rustyindie_common::api::api_service::ApiService;
use rustyindie_common::api::models::LoginResponse;
use rustyindie_common::components::page_not_found::PageNotFound;
use crate::pages::administrator::post::{manage_post::ManagePosts, add_edit_post::AddEditPost};
use crate::pages::administrator::{mail::Mail, admin::Admin};


#[component]
pub fn App() -> impl IntoView {
    let (api_service, _) = create_signal(ApiService::new());
    provide_context(api_service);

    let login_resp = create_signal::<Option<LoginResponse>>(None);    
    provide_context(login_resp);
    provide_meta_context();
    
    view! {
        <Router>
            <Title text="RustyIndie Admin" />
            <main>
                <Routes>
                    <Route path="/" view=Admin>
                        <Route path="/mail" view=Mail />
                        <Route path="/add_edit" view=AddEditPost />
                        <Route path="/add_edit/:post_id" view=AddEditPost />
                        <Route path="/mngpost" view=ManagePosts />
                        <Route path="/*" view=PageNotFound />
                    </Route>                    
                    <Route path="/*" view=PageNotFound />
                </Routes>
            </main>
        </Router>
        <div id="portal_root"></div>
    }
}