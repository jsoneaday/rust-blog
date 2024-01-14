pub mod common {
    pub mod authentication {
        pub mod auth_service;
    }    
    pub mod utils {
        pub mod datetime_utils;
        pub mod fs_utils;
        pub mod rand_utils;
    }
    pub mod repository {
        pub mod base;
        pub mod error;        
    }
}
pub mod routes {
    pub mod app_state;
    pub mod base_model;
    pub mod route_utils;
    pub mod stripped_down_error;
}