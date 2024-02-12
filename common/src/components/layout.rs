use leptos::*;

#[component]
pub fn Layout(single_column: bool, children: Children) -> impl IntoView {
    let col_class = move || {
        if single_column {
            "home-single-col"
        } else {
            "home-double-col"
        }
    };

    view! {
        <div class=col_class>            
            {children()}
        </div>
    }
}