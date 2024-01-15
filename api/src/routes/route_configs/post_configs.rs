use actix_web::web::{ServiceConfig, self};

pub fn post_configs(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/post")
    );
}