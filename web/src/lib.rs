pub mod pages {
    pub mod home;
    pub mod admin;
}
pub mod common {
    pub mod testing_utils {
        pub mod fake_data;
    }
    pub mod utils {
        pub mod md_to_html_util;
    }
    pub mod components {
        pub mod layout;
        pub mod post {
            pub mod post_detail;
            pub mod post_preview;
        }        
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