use actix_web::web::{ServiceConfig, self};
use crate::{common::{authentication::auth_service::AuthService, repository::base::DbRepo}, routes::authentication::routes::login};

pub fn admin_configs(cfg: &mut ServiceConfig) {
    cfg.service(
        web::resource("/user")            
    ).service(
        web::resource("/login")
            .route(web::post().to(login::<DbRepo, AuthService>))   
    );
}