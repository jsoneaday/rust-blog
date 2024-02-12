use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use rustyindie_common::api::api_service::ApiService;
use rustyindie_common::api::models::LoginResponse;
use rustyindie_common::components::page_not_found::PageNotFound;
use crate::pages::home::home::Home;
use crate::pages::home::individual_post::IndividualPost;


#[component]
pub fn App() -> impl IntoView {
    let (api_service, _) = create_signal(ApiService::new());
    provide_context(api_service);
    let login_resp_signal = create_signal::<Option<LoginResponse>>(None);
    provide_context(login_resp_signal);
    provide_meta_context();
    
    view! {
        <Router>
            <Title formatter=|text| format!("RustyIndie {text}") />
            <main>
                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/post/:post_id" view=IndividualPost />  
                    <Route path="/*" view=PageNotFound />               
                </Routes>
            </main>
        </Router>
        <div id="portal_root"></div>
    }
}