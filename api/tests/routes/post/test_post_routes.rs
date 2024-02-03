use fake::{faker::lorem::en::Sentence, Fake};
use rust_blog_api::{
    common::{authentication::auth_service::{AuthService, STANDARD_ACCESS_TOKEN_EXPIRATION}, repository::base::{DbRepo, Repository}}, 
    common_test::fixtures::{get_app_data, get_fake_httprequest_with_bearer_token}, 
    routes::{base_model::PagingModel, post::{models::NewPost, routes::{create_post, get_post, get_posts, get_post_previews, delete_post}}}
    
};
use actix_web::web::{Path, Json};

#[tokio::test]
async fn test_create_post_completes_successfully() {
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

/// I don't want to create unique databases for each test (maybe I'll do that once more tests are written),
/// for now I create a specific post and make sure that at least that specific post comes back
#[tokio::test]
async fn test_get_post_previews_returns_correct_previews() {
    let repo = DbRepo::init().await;
    let auth_service = AuthService;
    let app_data = get_app_data(repo, auth_service).await;
    let user_name = "dave";

    let req = get_fake_httprequest_with_bearer_token(user_name.to_string(), &app_data.auth_keys.encoding_key, "/v1/post", 1, Some(STANDARD_ACCESS_TOKEN_EXPIRATION));

    let created_post_res = create_post(app_data.clone(), Json(NewPost {
        title: Sentence(1..2).fake::<String>(),
        message: Sentence(3..5).fake::<String>(),
        admin_id: 1
    }), req).await;
    let post_id = created_post_res.unwrap().id;

    let get_post_res = get_post_previews(app_data, Path::from(PagingModel {
        page_size: 10,
        last_offset: 0
    }))
    .await;

    assert!(get_post_res.is_ok());
    assert!(get_post_res.unwrap().0.iter().find(|post| {
        post.id == post_id
    }).is_some());
}

#[tokio::test]
async fn test_get_posts_returns_correct_posts() {
    let repo = DbRepo::init().await;
    let auth_service = AuthService;
    let app_data = get_app_data(repo, auth_service).await;
    let user_name = "dave";

    let req = get_fake_httprequest_with_bearer_token(user_name.to_string(), &app_data.auth_keys.encoding_key, "/v1/post", 1, Some(STANDARD_ACCESS_TOKEN_EXPIRATION));

    let created_post_res = create_post(app_data.clone(), Json(NewPost {
        title: Sentence(1..2).fake::<String>(),
        message: Sentence(3..5).fake::<String>(),
        admin_id: 1
    }), req).await;
    let post_id = created_post_res.unwrap().id;

    let get_post_res = get_posts(app_data, Path::from(PagingModel {
        page_size: 10,
        last_offset: 0
    }))
    .await;

    assert!(get_post_res.is_ok());
    assert!(get_post_res.unwrap().0.iter().find(|post| {
        post.id == post_id
    }).is_some());
}

#[tokio::test]
async fn test_get_post_returns_correct_post() {
    let repo = DbRepo::init().await;
    let auth_service = AuthService;
    let app_data = get_app_data(repo, auth_service).await;
    let user_name = "dave";

    let req = get_fake_httprequest_with_bearer_token(user_name.to_string(), &app_data.auth_keys.encoding_key, "/v1/post", 1, Some(STANDARD_ACCESS_TOKEN_EXPIRATION));

    let created_post_res = create_post(app_data.clone(), Json(NewPost {
        title: Sentence(1..2).fake::<String>(),
        message: Sentence(3..5).fake::<String>(),
        admin_id: 1
    }), req).await;
    let post_id = created_post_res.unwrap().id;

    let get_post_res = get_post(app_data, Path::from(post_id)).await;

    assert!(get_post_res.is_ok());
    assert!(get_post_res.unwrap().unwrap().id == post_id);
}

#[tokio::test]
async fn test_delete_post_does_deletion() {
    let repo = DbRepo::init().await;
    let auth_service = AuthService;
    let app_data = get_app_data(repo, auth_service).await;
    let user_name = "dave";

    let req = get_fake_httprequest_with_bearer_token(user_name.to_string(), &app_data.auth_keys.encoding_key, "/v1/post", 1, Some(STANDARD_ACCESS_TOKEN_EXPIRATION));

    let created_post_res = create_post(app_data.clone(), Json(NewPost {
        title: Sentence(1..2).fake::<String>(),
        message: Sentence(3..5).fake::<String>(),
        admin_id: 1
    }), req).await;
    let post_id = created_post_res.unwrap().id;

    let delete_post_res = delete_post(app_data.clone(), Path::from(post_id)).await;
    assert!(delete_post_res.is_ok());
    assert!(delete_post_res.unwrap() == ());

    let get_post_res = get_post(app_data, Path::from(post_id)).await;
    assert!(get_post_res.is_ok());
    assert!(get_post_res.unwrap().is_none());
}