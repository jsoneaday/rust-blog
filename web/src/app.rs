use leptos::*;
use leptos_router::*;
use crate::common::api::api_service::ApiService;
use crate::pages::administrator::post::{manage_post::ManagePosts, add_post::AddPost};
use crate::pages::home::home::Home;
use crate::pages::administrator::{mail::Mail, admin::Admin};
use crate::pages::page_not_found::PageNotFound;

#[component]
pub fn App() -> impl IntoView {
    provide_context(create_signal(ApiService::new()));
    
    view! {
        <Router>
            <main>
                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/admin" view=Admin>
                        <Route path="/mail" view=Mail />
                        <Route path="/addpost" view=AddPost />
                        <Route path="/mngpost" view=ManagePosts />
                        <Route path="" view=PageNotFound />
                    </Route>                    
                </Routes>
            </main>
        </Router>
        <div id="portal_root"></div>
    }
}