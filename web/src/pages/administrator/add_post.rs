use leptos::*;
use leptos::logging::log;
use leptos_router::*;

#[component]
pub fn AddPost() -> impl IntoView {
    let (title, set_title) = create_signal("".to_string());
    let (content, set_content) = create_signal("".to_string());

    view! {
        <div class="home-content">
            <h2>"Add Post"</h2>
            <Form method="post" action="/dosomething">
                <section class="form-section">
                    <label for="title">
                        "Title"                    
                    </label>
                    <input 
                        type="text" 
                        id="title"
                        on:input=move |ev| {
                            log!("target value: {}", event_target_value(&ev));
                            set_title(event_target_value(&ev));
                        } 
                        prop:value=title
                    />
                </section>
                <section class="form-section" style="height: 600px">
                    <label for="content">
                        "Content"                    
                    </label>
                    <textarea />
                </section>
            </Form>
        </div>
    }
}