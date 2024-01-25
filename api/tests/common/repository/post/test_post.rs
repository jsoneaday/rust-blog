use rust_blog_api::{
    common::{repository::{base::{DbRepo, Repository}, administrator::repo::InsertAdminisratorFn, post::repo::{InsertPostFn, QueryPostsFn}}, authentication::auth_service::AuthService}, 
    common_test::fixtures::get_app_data
};
use fake::{Fake, faker::{internet::en::{Username, SafeEmail, Password}, lorem::en::Sentence}};

#[tokio::test]
async fn test_query_posts_return_correct_posts() {
    let repo = DbRepo::init().await;
    let app_data = get_app_data(repo, AuthService).await;
    let title = Sentence(1..2).fake::<String>();
    let message = Sentence(1..5).fake::<String>();

    let entity_admin_result = app_data.repo.insert_administrator(Username().fake::<String>(), SafeEmail().fake::<String>(), Password(5..10).fake::<String>()).await.unwrap();
    _ = app_data.repo.insert_post(title.clone(), message.clone(), entity_admin_result.id).await.unwrap();
    let post_result = app_data.repo.query_posts(10, 0).await.unwrap();
    
    assert!(post_result.len() > 0);
    assert!(post_result.first().unwrap().message == message);
    assert!(post_result.first().unwrap().admin_id == entity_admin_result.id);
}

#[tokio::test]
async fn test_insert_post_returns_valid_admin() {
    let repo = DbRepo::init().await;
    let app_data = get_app_data(repo, AuthService).await;
    let title = Sentence(1..2).fake::<String>();
    let message = Sentence(1..5).fake::<String>();

    let entity_admin_result = app_data.repo.insert_administrator(Username().fake::<String>(), SafeEmail().fake::<String>(), Password(5..10).fake::<String>()).await.unwrap();
    let entity_post_result = app_data.repo.insert_post(title.clone(), message.clone(), entity_admin_result.id).await.unwrap();
    
    assert!(entity_post_result.id > 0);
}