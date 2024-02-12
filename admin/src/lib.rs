pub mod pages {
    pub mod administrator {
        pub mod post {
            pub mod add_edit_post;
            pub mod manage_post;
        }
        pub mod admin;
        pub mod mail;        
    }
}
pub mod app;

use leptos::*;
use crate::app::App;

pub fn run() {
    mount_to_body(|| {
        view! { <App/> }
    });
}