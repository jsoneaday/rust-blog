pub mod api {
    pub mod api_service;
    pub mod models;
}
pub mod utils {
    pub mod date_time;
    pub mod fs_utils;
    pub mod markdown_to_html;
}
pub mod components {
    pub mod layout;
    pub mod modal;
    pub mod page_not_found;
    pub mod authentication {
        pub mod login;
    }
    pub mod post {
        pub mod post_detail;
        pub mod post_preview;
        pub mod list_post_previews;
    }
}