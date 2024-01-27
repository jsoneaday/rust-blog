use leptos::*;
use leptos::logging::log;
use crate::common::api::models::{LoginResponse, NewPost};
use crate::common::api::api_service::ApiService;

#[component]
pub fn AddPost() -> impl IntoView {
    let (title, set_title) = create_signal("".to_string());
    let (content, set_content) = create_signal("".to_string());
    let api_service = expect_context::<ReadSignal<ApiService>>();
    let (login_resp, _) = expect_context::<(ReadSignal<Option<LoginResponse>>, WriteSignal<Option<LoginResponse>>)>();
    
    let submit_post = create_action(move |new_post: &NewPost| {
        let input = new_post.clone();
        async move { 
            match login_resp() {
                Some(login_result) => {
                    let id_res = api_service.get_untracked().create_post(&input, login_result.access_token).await;
                    match id_res { 
                        Ok(output_id) => log!("create_post success: {:?}",  output_id),
                        Err(e) => log!("create_post failed: {:?}", e)
                    };  
                },
                None => {
                    // todo: add notification that user must login!
                    log!("create_post failed: user must login first");
                }
            }                      
        }
    });

    view! {
        <div class="home-content">
            <h2>"Add Post"</h2>
            <form on:submit=move |ev| {
                ev.prevent_default();
                submit_post.dispatch(NewPost { title: title(), message: content(), admin_id: 1 });
            }>
                <section class="form-section">
                    <label for="title">
                        "Title"                    
                    </label>
                    <input 
                        type="text" 
                        id="title"
                        name="title"
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
                    <textarea
                        id="content"
                        name="content"
                        prop:value=move || content()
                        on:input=move |ev| {
                            set_content(event_target_value(&ev));
                        }
                    >
                        {untrack(move || content())}
                    </textarea>
                </section>
                <section class="form-section">
                    <button type="submit" class="primary-btn" >"Post"</button>                    
                </section>
            </form>
        </div>
    }
}