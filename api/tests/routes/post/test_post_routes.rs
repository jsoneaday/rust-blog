use fake::{faker::lorem::en::Sentence, Fake};
use rust_blog_api::{
    common::{authentication::auth_service::{AuthService, STANDARD_ACCESS_TOKEN_EXPIRATION}, repository::base::{DbRepo, Repository}}, 
    common_test::fixtures::{get_app_data, get_fake_httprequest_with_bearer_token}, 
    routes::post::{models::NewPost, routes::create_post}
    
};
use actix_web::web::Json;

#[tokio::test]
async fn test_post_route_create_post_completes_successfully() {
    let repo = DbRepo::init().await;
    let auth_service = AuthService;
    let app_data = get_app_data(repo, auth_service).await;
    let user_name = "dave";

    let req = get_fake_httprequest_with_bearer_token(user_name.to_string(), &app_data.auth_keys.encoding_key, "/v1/authentication", 1, Some(STANDARD_ACCESS_TOKEN_EXPIRATION));

    let post_res = create_post(app_data, Json(NewPost {
        title: Sentence(1..2).fake::<String>(),
        message: Sentence(3..5).fake::<String>(),
        admin_id: 1
    }), req).await;

    assert!(post_res.is_ok());
    assert!(post_res.unwrap().id > 0);
}