use leptos::*;
use leptos::logging::log;

#[component]
pub fn Login() -> impl IntoView {
    let (email, set_email) = create_signal("".to_string());
    let (password, set_password) = create_signal("".to_string());
        
    view! {
        <form>
            <section class="form-section">
                <label for="email">
                    "Email"                    
                </label>
                <input 
                    type="text" 
                    id="email"
                    name="email"
                    on:input=move |ev| {
                        log!("email value: {}", event_target_value(&ev));
                        set_email(event_target_value(&ev));
                    } 
                    prop:value=email
                />
            </section>
            <section class="form-section">
                <label for="password">
                    "Password"                    
                </label>
                <input 
                    type="text" 
                    id="password"
                    name="password"
                    on:input=move |ev| {
                        log!("password value: {}", event_target_value(&ev));
                        set_password(event_target_value(&ev));
                    } 
                    prop:value=password
                />
            </section>
            <section class="form-section">
                <button type="submit" class="primary-btn" >"Login"</button>
            </section>
        </form>
    }
}