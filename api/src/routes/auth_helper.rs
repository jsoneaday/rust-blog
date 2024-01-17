use actix_http::body::to_bytes;
use actix_web::{web::Data, HttpRequest, HttpResponse};
use jsonwebtoken::DecodingKey;
use log::{info, error};
use crate::{common::{repository::base::Repository, authentication::auth_service::{Authenticator, decode_token, Claims}}, routes::route_utils::get_header_strings};
use super::app_state::AppState;

pub async fn check_is_authenticated<T: Repository, U: Authenticator>(
    app_data: Data<AppState<T, U>>,
    req: HttpRequest
) -> bool {
    #[allow(unused)]
    let mut user_name: String = "".to_string();
    
    let headers = get_header_strings(req.headers());
    info!("headers {:?}", headers);
    let is_authenticated_result = app_data.auth_service.is_authenticated(user_name, headers, &app_data.auth_keys.decoding_key).await;
    match is_authenticated_result {
        Ok(result) => match result {
            true => {
                info!("Successfully authorized");
                true
            },
            false => {
                info!("Failed authorization");
                false
            }
        },
        Err(_) => {
            error!("Authorization attempt failed");
            false
        }
    }
}

pub async fn get_claims_from_token_body(decoding_key: &DecodingKey, httpresponse: HttpResponse) -> Claims {
    let (_res, body) = httpresponse.into_parts();
    let new_token_bytes = to_bytes(body).await.unwrap();
    let new_token_str = String::from_utf8_lossy(&new_token_bytes);
    decode_token(&new_token_str, decoding_key)
}