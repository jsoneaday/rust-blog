use leptos::*;
use leptos_router::*;
use crate::pages::home::Home;
use crate::pages::admin::Admin;

#[component]
pub fn App() -> impl IntoView {

    view! {
        <Router>
            <nav></nav>
            <main>
                <Routes>
                    <Route path="/" view=Home />
                    <Route path="/admin" view=Admin />
                </Routes>
            </main>
        </Router>
    }
}