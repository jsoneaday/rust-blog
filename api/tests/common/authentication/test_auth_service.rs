use rust_blog_api::common::repository::base::{Repository, DbRepo};
use rust_blog_api::common::authentication::auth_service::{AuthService, Authenticator};
use rust_blog_api::common_test::fixtures::{get_app_data, get_fake_httprequest_with_bearer_token};
use rust_blog_api::routes::route_utils::get_header_strings;

#[tokio::test]
async fn test_is_authenticated() {    
    let repo = DbRepo::init().await;
    let auth_service = AuthService;
    let app_data = get_app_data(repo, auth_service).await;
    let user_name = "jon".to_string();
    
    let req = get_fake_httprequest_with_bearer_token(
        user_name.clone(), &app_data.auth_keys.encoding_key, "/v1/developer", 1, Some(60*2), None
    );
    let headers = get_header_strings(req.headers());

    let result = app_data.auth_service.is_authenticated(user_name, headers, &app_data.auth_keys.decoding_key).await.unwrap();

    assert!(result == true);
}