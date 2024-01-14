use crate::{common::{repository::base::Repository, authentication::auth_service::{Authenticator, init_auth_keys}}, routes::app_state::AppState};


pub async fn get_app_data<T: Repository, U: Authenticator>(repo: T, auth_service: U) -> actix_web::web::Data<AppState<T, U>> {
    actix_web::web::Data::new(AppState { repo, auth_service, auth_keys: init_auth_keys().await })
}