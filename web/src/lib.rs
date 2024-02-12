pub mod pages {
    pub mod home {
        pub mod home;
        pub mod individual_post;
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