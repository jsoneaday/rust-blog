use leptos::*;
use leptos::logging::log;
use leptos_router::{Params, use_params};
use rustyindie_common::api::models::{LoginResponse, NewPost, UpdatePost};
use rustyindie_common::api::api_service::ApiService;

#[derive(Params, PartialEq)]
struct AddEditPostParams {
    post_id: Option<i64>
}

const POST: &str = "Post";
const EDIT: &str = "Edit";

#[component]
pub fn AddEditPost() -> impl IntoView {
    let add_edit_params = use_params::<AddEditPostParams>();
    let post_id = move || {
        add_edit_params.with(|params| {
            params
            .as_ref()
            .map(|param| param.post_id)
            .unwrap_or_default()
        })
    };
    let (title, set_title) = create_signal("".to_string());
    let (content, set_content) = create_signal("".to_string());
    let api_service = expect_context::<ReadSignal<ApiService>>();
    let (login_resp, _) = expect_context::<(ReadSignal<Option<LoginResponse>>, WriteSignal<Option<LoginResponse>>)>();

    _ = create_resource(post_id, move |id| async move {
        if let None = login_resp() {
            return None;
        }
        if let None = id {
            return None;
        }
        
        let post_res = api_service.get_untracked().get_post(id.unwrap_or_default()).await;
        match post_res {
            Ok(opt_post) => match opt_post {
                Some(post) => {
                    set_title(post.title.clone());
                    set_content(post.message.clone());
                    Some(post)
                },
                None => None
            },
            Err(_) => {
                log!("Error getting post");
                None
            }
        }
    });
       
    let submit_new_post = create_action(move |new_post: &NewPost| {
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

    let submit_update_post = create_action(move |update_post: &UpdatePost| {
        let input = update_post.clone();
        async move { 
            match login_resp() {
                Some(login_result) => {
                    let result = api_service.get_untracked().update_post(&input, login_result.access_token).await;
                    match result { 
                        Ok(_resp) => log!("update_post success"),
                        Err(e) => log!("update_post failed: {:?}", e)
                    };  
                },
                None => {
                    // todo: add notification that user must login!
                    log!("update_post failed: user must login first");
                }
            }                      
        }
    });

    let disable_post_submit = move || match login_resp() {
        Some(_login_result) => false,
        None => true
    };

    let submit_btn_label = move || {
        if let Some(_id) = post_id() {
            EDIT
        } else {
            POST
        }
    };

    view! {
        <div class="home-content">
            <h2>"Add Post"</h2>
            <form on:submit=move |ev| {
                ev.prevent_default();
                if let None = post_id() {
                    submit_new_post.dispatch(NewPost { title: title(), message: content(), admin_id: 1 });
                } else {
                    submit_update_post.dispatch(UpdatePost { post_id: post_id().unwrap(), admin_id: 1, title: title(), message: content() });
                }
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
                            set_title(event_target_value(&ev));
                        } 
                        prop:value=title
                        style="width: 100%"
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
                    <button prop:disabled=disable_post_submit type="submit" class="primary-btn" >{submit_btn_label}</button>                    
                </section>
            </form>
        </div>
    }
}