use leptos::logging::log;
use leptos::*;
use leptos_router::A;
use leptos_meta::*;
use crate::common::components::post::post_preview::PostPreviewParams;
use crate::common::components::{layout::Layout, post::post_preview::PostPreview};
use crate::common::api::api_service::ApiService;
use crate::common::utils::date_time::convert_datetime_long_readable;
use crate::common::utils::fs_utils::get_file_buffer;

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use js_sys::{BigInt, Object};

#[wasm_bindgen(module = "/wasm/main.mjs")]
extern "C" {
    pub type WebIrys;

    #[wasm_bindgen(constructor)]
    pub fn new(params: &JsValue) -> WebIrys;

    #[wasm_bindgen(method, js_name = getPrice)]
    pub async fn get_price(this: &WebIrys, bytes: i64) -> JsValue;

    #[wasm_bindgen(js_name = getIrysInstance)]
    pub async fn get_irys_instance(url: String, token: String) -> JsValue;
}

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug)]
pub struct IrysConstructorConfig {
    url: Option<String>,
    token: Option<String>
}

#[wasm_bindgen]
pub fn connect_to_irys(url: Option<String>, token: Option<String>) -> WebIrys {
    let con_config = IrysConstructorConfig {
        url, token
    };
    let configs = serde_wasm_bindgen::to_value(&con_config).unwrap();

    WebIrys::new(&configs)
}

#[component]
pub fn Home() -> impl IntoView {
    let setup_wasm = create_action(|_: &()| async move {
        // let irys = connect_to_irys(
        //     Some("https://devnet.irys.xyz".to_string()), 
        //     Some("solana".to_string())
        // );
        // log!("irys price {:?}", js_sys::BigInt::from(irys.get_price(10000000).await));
        let irys = get_irys_instance("https://devnet.irys.xyz".to_string(), "solana".to_string()).await;
        log!("irys {:?}", irys);
    });

    let (last_offset, _set_last_offset) = create_signal(0);    
    let api_service = expect_context::<ReadSignal<ApiService>>();
  
    let posts = create_resource(last_offset, move |offset| async move {
        setup_wasm.dispatch(());
        let result = api_service.get_untracked().get_latest_posts(offset).await;
        match result {
            Ok(data) => {
                data
            },
            Err(e) => {
                log!("Failed to get post data: {}", e);
                vec![]
            }
        }
    });

    view! {
        <Layout single_column=false>
            <Title text="- Blog Posts" />
            <div class="home-menu">
                <h1>"Rust Indie Dev"</h1>
                <h2 style="margin-top: -0.4em">"Indie Development with Rust"</h2>
                <strong>"David Choi"</strong>
                <b style="margin-top: 0.5em"><i><small>"contact me"</small></i></b>
                <div style="margin-top: 1em">
                    <A href="https://github.com/jsoneaday/rust-blog" target="_blank">
                        <small>"This app is built entirely with Rust: Leptos, Actix Web"</small>
                    </A>
                </div>                
            </div>
            <div class="home-content">
                <ul>
                    <For
                        each=move || match posts() {
                            None => vec![],
                            Some(data) => data
                        }
                        key=|post| post.id
                        children=move |post| {
                            view! {
                                <div style="margin-bottom: 6em">
                                    <div></div>
                                    <PostPreview post=PostPreviewParams {
                                        id: post.id,
                                        updated_at: convert_datetime_long_readable(post.updated_at),
                                        title: post.title.to_string(),
                                        content: post.message.to_string()
                                    } />
                                </div>
                            }
                        }
                    />
                </ul>
            </div>
        </Layout>
    }
}