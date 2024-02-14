use leptos::*;
use leptos_router::use_location;

#[component]
pub fn PageNotFound() -> impl IntoView {
    let location = use_location();
    view! {
        <div>
            {move || {
                format!("Error 404, {} does not exist!", location.pathname.get())
            }}
        </div>
    }
}