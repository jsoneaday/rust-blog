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

use crate::routes::app_state::AppState;
use std::env;
use dotenv::dotenv;
use openssl::ssl::{SslAcceptorBuilder, SslAcceptor, SslMethod, SslFiletype};

#[allow(unused)]
fn ssl_builder() -> SslAcceptorBuilder {
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("ssl/key.pem", SslFiletype::PEM)
        .expect("failed to open/read key.pem");
    builder.set_certificate_chain_file("ssl/cert.pem")
        .expect("failed to open/read cert.pem");
    builder
}

