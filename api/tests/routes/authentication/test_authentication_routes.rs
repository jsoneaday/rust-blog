use actix_web::web::Json;
use fake::{faker::internet::en::Username, Fake};
use rustyindie_api::{
    common::{authentication::auth_service::{decode_token, AuthService, STANDARD_ACCESS_TOKEN_EXPIRATION}, repository::base::{DbRepo, Repository}}, 
    routes::{auth_helper::{get_access_token_from_login_resp_httpresponse, get_access_token_from_str_body_httpresponse}, authentication::{models::{LoginCredential, RefreshToken}, routes::{login, refresh_access_token}}, route_utils::get_header_strings}
};
use rustyindie_api::common_test::fixtures::{get_app_data, get_fake_httprequest_with_bearer_token};

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

    let token = get_access_token_from_str_body_httpresponse(httpresponse).await;
    let claims = decode_token(&token, &app_data.auth_keys.decoding_key);

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

    let token = get_access_token_from_login_resp_httpresponse(login_resp).await;
    let claims = decode_token(&token, &app_data.auth_keys.decoding_key);

    assert!(claims.sub == user_name);
    assert!(claims.exp >= STANDARD_ACCESS_TOKEN_EXPIRATION as usize);
}