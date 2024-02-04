pub mod pages {
    pub mod administrator {
        pub mod post {
            pub mod add_edit_post;
            pub mod manage_post;
        }
        pub mod admin;
        pub mod mail;        
    }
    pub mod page_not_found;
}
pub mod common {
    pub mod testing_utils {
        pub mod fake_data;
    }
    pub mod utils {
        pub mod markdown_to_html;
        pub mod date_time;
    }
    pub mod components {
        pub mod layout;
        pub mod post {
            pub mod post_preview;
            pub mod post_detail;
        }        
        pub mod authentication {
            pub mod login;
        }
        pub mod modal;
    }
    pub mod api {
        pub mod api_service;
        pub mod models;
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