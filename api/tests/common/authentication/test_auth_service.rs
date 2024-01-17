use fake::Fake;
use fake::faker::internet::en::Username;
use rust_blog_api::common::repository::base::{Repository, DbRepo};
use rust_blog_api::common::authentication::auth_service::{AuthService, Authenticator, STANDARD_ACCESS_TOKEN_EXPIRATION, init_auth_keys, get_token, decode_token};
use rust_blog_api::common_test::fixtures::{get_app_data, get_fake_httprequest_with_bearer_token};
use rust_blog_api::routes::route_utils::get_header_strings;

#[tokio::test]
async fn test_init_auth_keys_does_not_panic() {
    init_auth_keys().await;
}

#[tokio::test]
async fn test_get_token_returns_valid_token() {
    let auth_keys = init_auth_keys().await;
    let token = get_token(Username().fake::<String>(), &auth_keys.encoding_key, Some(STANDARD_ACCESS_TOKEN_EXPIRATION));

    assert!(token.len() > 0);
}

#[tokio::test]
async fn test_decode_token_returns_valid_claims() {
    let user_name = Username().fake::<String>();
    
    let auth_keys = init_auth_keys().await;
    let token = get_token(user_name.clone(), &auth_keys.encoding_key, Some(STANDARD_ACCESS_TOKEN_EXPIRATION));
    let claims = decode_token(&token, &auth_keys.decoding_key);

    assert!(claims.sub == user_name);
    assert!(claims.exp >= STANDARD_ACCESS_TOKEN_EXPIRATION as usize);
}

#[tokio::test]
async fn test_is_authenticated_returns_correct_boolean() {    
    let repo = DbRepo::init().await;
    let auth_service = AuthService;
    let app_data = get_app_data(repo, auth_service).await;
    let user_name = Username().fake::<String>();
    
    let req = get_fake_httprequest_with_bearer_token(
        user_name.clone(), &app_data.auth_keys.encoding_key, "/v1/administrator", 1, Some(STANDARD_ACCESS_TOKEN_EXPIRATION)
    );
    let headers = get_header_strings(req.headers());

    let result = app_data.auth_service.is_authenticated(user_name, headers, &app_data.auth_keys.decoding_key).await.unwrap();

    assert!(result == true);
}

