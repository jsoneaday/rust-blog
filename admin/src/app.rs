use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::common::api::api_service::ApiService;
use crate::common::api::models::LoginResponse;
use crate::pages::administrator::post::{manage_post::ManagePosts, add_post::AddPost};
use crate::pages::administrator::{mail::Mail, admin::Admin};
use crate::pages::page_not_found::PageNotFound;

#[component]
pub fn App() -> impl IntoView {
    let (api_service, _) = create_signal(ApiService::new());
    provide_context(api_service);
    let login_resp_signal = create_signal::<Option<LoginResponse>>(None);
    provide_context(login_resp_signal);
    provide_meta_context();
    
    view! {
        <Router>
            <Title formatter=|_text| format!("RustyIndie Admin") />
            <main>
                <Routes>
                    <Route path="/admin" view=Admin>
                        <Route path="/mail" view=Mail />
                        <Route path="/addpost" view=AddPost />
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