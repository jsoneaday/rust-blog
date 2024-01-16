use rust_blog_api::{
    common::{repository::{base::{DbRepo, Repository}, administrator::{repo::{InsertAdminisratorFn, QueryAdministratorFn, AuthenticateDbFn}, models::AuthenticateResult}}, authentication::auth_service::AuthService}, 
    common_test::fixtures::get_app_data
};
use fake::{Fake, faker::internet::en::{Username, SafeEmail, Password}};

#[tokio::test]
async fn test_insert_administrator_returns_valid_admin() {
    let repo = DbRepo::init().await;
    let app_data = get_app_data(repo, AuthService).await;

    let entity_result = app_data.repo.insert_administrator(Username().fake::<String>(), SafeEmail().fake::<String>(), Password(5..10).fake::<String>()).await.unwrap();
    
    assert!(entity_result.id > 0);
}

#[tokio::test]
async fn test_query_administrator_returns_correct_admin() {
    let repo = DbRepo::init().await;
    let app_data = get_app_data(repo, AuthService).await;
    let user_name = Username().fake::<String>();
    let email = SafeEmail().fake::<String>();
    let password = Password(5..10).fake::<String>();

    let entity_result = app_data.repo.insert_administrator(user_name.clone(), email.clone(), password.clone()).await.unwrap();
    let admin = app_data.repo.query_administrator(entity_result.id).await.unwrap().unwrap();

    assert!(admin.id == entity_result.id);
    assert!(admin.user_name == user_name);
    assert!(admin.email == email);
    assert!(admin.password == password);
}

#[tokio::test]
async fn test_authenticate_db_returns_correct_authenticateresult() {
    let repo = DbRepo::init().await;
    let app_data = get_app_data(repo, AuthService).await;
    let user_name = Username().fake::<String>();
    let email = SafeEmail().fake::<String>();
    let password = Password(5..10).fake::<String>();

    let entity_result = app_data.repo.insert_administrator(user_name.clone(), email.clone(), password.clone()).await.unwrap();
    let auth_result = app_data.repo.authenticate_db(email, password).await.unwrap();

    assert!(auth_result == AuthenticateResult::Success { id: entity_result.id });
}