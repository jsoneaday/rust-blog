use actix_web::web::Json;
use fake::{faker::internet::en::Username, Fake};
use rust_blog_api::{
    common::{repository::base::{DbRepo, Repository}, authentication::auth_service::{AuthService, STANDARD_ACCESS_TOKEN_EXPIRATION}}, 
    routes::{authentication::{routes::{login, refresh_access_token}, models::{LoginCredential, RefreshToken}}, auth_helper::get_claims_from_token_body, route_utils::get_header_strings}
};
use rust_blog_api::common_test::fixtures::{get_app_data, get_fake_httprequest_with_bearer_token};

#[tokio::test]
async fn test_refresh_access_token_is_valid() {
    let repo = DbRepo::init().await;
    let auth_service = AuthService;
    let app_data = get_app_data(repo, auth_service).await;
    let user_name = Username().fake::<String>();

    let req = get_fake_httprequest_with_bearer_token(user_name.to_string(), &app_data.auth_keys.encoding_key, "/v1/authentication", 1, Some(STANDARD_ACCESS_TOKEN_EXPIRATION));
    let headers = get_header_strings(req.headers());
    let bearer_header = headers
        .iter()
        .find(|header| {
            let name = header.0;
            name.to_lowercase() == "authorization"
        }).unwrap();
    let value = bearer_header.1;
    let bearer = value.split(' ').collect::<Vec<&str>>();
    let old_token = bearer.get(1).unwrap().to_string();
    
    let httpresponse = refresh_access_token(app_data.clone(), Json(RefreshToken{
        old_token
    }), req).await;

    let claims = get_claims_from_token_body(&app_data.auth_keys.decoding_key, httpresponse).await;

    assert!(claims.sub == user_name);
    assert!(claims.exp >= STANDARD_ACCESS_TOKEN_EXPIRATION as usize);
}

#[tokio::test]
async fn test_login_logs_in_successfully() {
    let repo = DbRepo::init().await;
    let auth_service = AuthService;
    let app_data = get_app_data(repo, auth_service).await;
    let user_name = "dave";

    let login_resp = login(app_data.clone(), Json(LoginCredential {
        email: "dharric@live.com".to_string(),
        password: "123".to_string()
    })).await;

    let claims = get_claims_from_token_body(&app_data.auth_keys.decoding_key, login_resp).await;

    assert!(claims.sub == user_name);
    assert!(claims.exp >= STANDARD_ACCESS_TOKEN_EXPIRATION as usize);
}