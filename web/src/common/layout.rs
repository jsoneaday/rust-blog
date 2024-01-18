use leptos::*;

#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="layout-container home">
            {children()}
        </div>
    }
}