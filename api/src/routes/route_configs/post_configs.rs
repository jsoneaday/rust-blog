use actix_web::web::{ServiceConfig, self};
use crate::routes::post::routes::create_post;
use crate::common::{authentication::auth_service::AuthService, repository::base::DbRepo};

pub fn post_configs(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/post").route(web::post().to(create_post::<DbRepo, AuthService>))
    );
}