use leptos::*;
use leptos::logging::log;
use crate::api::api_service::ApiService;
use crate::api::models::{LoginCredential, LoginResponse};

#[component]
pub fn Login() -> impl IntoView {
    let (email, set_email) = create_signal("dharric@live.com".to_string());
    let (password, set_password) = create_signal("123".to_string());
    let api_service = expect_context::<ReadSignal<ApiService>>();
    let (_, set_login_resp) = expect_context::<(ReadSignal<Option<LoginResponse>>, WriteSignal<Option<LoginResponse>>)>();
    let submit_post = create_action(move |credentials: &LoginCredential| {
        let credentials = credentials.clone();

        async move {
            let login_result = api_service.get_untracked().login(&credentials).await;
            match login_result {
                Ok(login_resp) => {
                    set_login_resp(Some(login_resp.clone()));
                    log!("login success, response: {:?}", login_resp);
                },
                Err(e) => log!("login failed: {}", e)
            };
        }
    });
        
    view! {
        <form
            on:submit=move |ev| {
                ev.prevent_default();

                submit_post.dispatch(LoginCredential {
                    email: email(),
                    password: password()
                });
            }
        >
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