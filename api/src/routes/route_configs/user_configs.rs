use actix_web::web::{ServiceConfig, self};

pub fn user_configs(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/user")
    );
}