use leptos::*;
use leptos::logging::log;
use crate::common::api::api_service::ApiService;
use crate::common::api::models::LoginCredential;

#[component]
pub fn Login() -> impl IntoView {
    let (email, set_email) = create_signal("dharric@live.com".to_string());
    let (password, set_password) = create_signal("123".to_string());
    let api_service = expect_context::<ReadSignal<ApiService>>();
    let (_auth_token, set_auth_token) = expect_context::<(ReadSignal<String>, WriteSignal<String>)>();
    let submit_post = create_action(move |credentials: &LoginCredential| {
        let credentials = credentials.clone();

        async move {
            let login_res = api_service.get_untracked().login(&credentials).await;
            match login_res {
                Ok(token) => {
                    set_auth_token(token.clone());
                    log!("login success, token: {}", token);
                },
                Err(e) => log!("login failed {}", e)
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