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
        pub mod administrator {
            pub mod models;
            pub mod repo;
        }
        pub mod post {
            pub mod models;
            pub mod repo;
        }
    }
}
pub mod routes {
    pub mod route_configs {
        pub mod user_configs;
        pub mod post_configs;
    }
    pub mod authentication {
        pub mod models;
        pub mod routes;
    }    
    pub mod app_state;
    pub mod base_model;
    pub mod route_utils;
    pub mod stripped_down_error;
}
pub mod common_test {
    pub mod fixtures;
}

use crate::routes::app_state::AppState;
use std::env;
use actix_cors::Cors;
use actix_web::{HttpServer, App, http::header, middleware::Logger, web};
use common::{repository::base::{DbRepo, Repository}, authentication::auth_service::{AuthService, init_auth_keys}};
use dotenv::dotenv;
use openssl::ssl::{SslAcceptorBuilder, SslAcceptor, SslMethod, SslFiletype};
use crate::routes::route_configs::post_configs::post_configs;
use crate::routes::route_configs::user_configs::user_configs;

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

pub async fn run() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    
    dotenv().ok();
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap().parse::<u16>().unwrap();
    
    let app_data = actix_web::web::Data::new(AppState{
        repo: DbRepo::init().await,
        auth_service: AuthService,
        auth_keys: init_auth_keys().await
    });    

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())     
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![
                        header::CONTENT_TYPE,
                        header::AUTHORIZATION,
                        header::ACCEPT, // todo: might not need this
                    ])
                    .supports_credentials()
                    .max_age(3600)
            )
            .service(
                web::scope("/v1")
                    .configure(user_configs)
                    .configure(post_configs)
            )
    })
    .bind((host, port)).expect("")
    // .bind_openssl((host, post), ssl_builder())?
    .run()
    .await
}