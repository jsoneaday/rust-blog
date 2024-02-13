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
        pub mod mail {
            pub mod models;
            pub mod repo;
        }
    }
}
pub mod routes {
    pub mod route_configs {
        pub mod admin_configs;
        pub mod post_configs;
    }
    pub mod authentication {
        pub mod models;
        pub mod routes;
    }
    pub mod post {
        pub mod models;
        pub mod routes;
    }
    pub mod mail {
        pub mod models;
        pub mod routes;
    }
    pub mod app_state;
    pub mod base_model;
    pub mod route_utils;
    pub mod stripped_down_error;
    pub mod auth_helper;
}
pub mod common_test {
    pub mod fixtures;
}

use crate::routes::app_state::AppState;
use std::env;
use std::fs::File;
use std::io::BufReader;
use actix_cors::Cors;
use actix_web::{HttpServer, App, http::header, middleware::Logger, web};
use log::error;
use rustls::{Certificate, PrivateKey, ServerConfig};
use rustls_pemfile::{certs, pkcs8_private_keys};
use common::{repository::base::{DbRepo, Repository}, authentication::auth_service::{AuthService, init_auth_keys}};
use dotenv::dotenv;
use crate::routes::route_configs::post_configs::post_configs;
use crate::routes::route_configs::admin_configs::admin_configs;

fn load_rustls_config() -> rustls::ServerConfig {
    let config = ServerConfig::builder();

    let cert_file = &mut BufReader::new(File::open("ssl/cert.pem").unwrap());
    let key_file = &mut BufReader::new(File::open("ssl/key.pem").unwrap());

    let cert_chain = certs(cert_file)
        .unwrap()
        .into_iter()
        .map(Certificate)
        .collect();

    let mut keys: Vec<PrivateKey> = pkcs8_private_keys(key_file)
        .unwrap()
        .into_iter()
        .map(PrivateKey)
        .collect();

    if keys.is_empty() {
        error!("could not locate pkcs 8 private keys");
        std::process::exit(1);
    }

    config
        .with_safe_default_cipher_suites()
        .with_safe_default_kx_groups()
        .with_safe_default_protocol_versions()
        .unwrap()
        .with_no_client_auth()
        .with_single_cert(cert_chain, keys.remove(0)).unwrap()
}

pub async fn run() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));
    
    dotenv().ok();
    let host = env::var("HOST").unwrap();
    let port = env::var("PORT").unwrap().parse::<u16>().unwrap();
    let allowed_web_url = env::var("ALLOWED_WEB_URL").unwrap();
    let allowed_admin_url = env::var("ALLOWED_ADMIN_URL").unwrap();
    
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
                    .allowed_origin(&allowed_admin_url)
                    .allowed_origin(&allowed_web_url)
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
                    .configure(admin_configs)
                    .configure(post_configs)
            )
    })
    // .bind((host, port)).expect("")
    .bind_rustls_021((host, port), load_rustls_config())?
    .run()
    .await
}