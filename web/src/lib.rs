pub mod pages {
    pub mod home;
    pub mod admin;
}
pub mod app;

use leptos::*;
use crate::app::App;

pub fn run() {
    mount_to_body(|| {
        view! { <App/> }
    });
}