use actix_http::header::HeaderValue;
use actix_web::{http::header, cookie::{time::Duration as ActixWebDuration, Cookie}};
use actix_web::{HttpRequest, test};
use jsonwebtoken::EncodingKey;
use serde::Serialize;

use crate::{common::{repository::base::Repository, authentication::auth_service::{Authenticator, init_auth_keys, get_token, REFRESH_TOKEN_LABEL, STANDARD_REFRESH_TOKEN_EXPIRATION}}, routes::app_state::AppState};

pub fn get_fake_httprequest_with_bearer_token(
    user_name: String,
    encoding_key: &EncodingKey, 
    url: &str, 
    parameter_data: impl Serialize, 
    token_expiration_duration: Option<i64>
) -> HttpRequest {
    let header_value_string = format!("Bearer {}", get_token(user_name.to_string(), encoding_key, token_expiration_duration));
    let header_value = HeaderValue::from_str(&header_value_string).unwrap();
    let req = test::TestRequest
        ::post()
        .append_header((header::AUTHORIZATION, header_value.clone()))
        .uri(url)
        .set_json(parameter_data);     
        
    let refresh_token = get_token(user_name, encoding_key, None);
    let refresh_cookie = Cookie::build(REFRESH_TOKEN_LABEL, refresh_token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(STANDARD_REFRESH_TOKEN_EXPIRATION, 0))
        .http_only(true)
        .secure(false)
        //.same_site(SameSite::Lax)
        .finish();

    req.cookie(refresh_cookie).to_http_request()
}

pub async fn get_app_data<T: Repository, U: Authenticator>(repo: T, auth_service: U) -> actix_web::web::Data<AppState<T, U>> {
    actix_web::web::Data::new(AppState { repo, auth_service, auth_keys: init_auth_keys().await })
}