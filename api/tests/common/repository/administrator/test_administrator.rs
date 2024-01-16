use rust_blog_api::{
    common::{repository::{base::{DbRepo, Repository}, administrator::repo::{InsertAdminisratorFn, QueryAdministratorFn}}, authentication::auth_service::AuthService}, 
    common_test::fixtures::get_app_data
};
use fake::{Fake, faker::internet::en::{Username, SafeEmail, Password}};

#[tokio::test]
async fn test_authenticate_db() {
    let repo = DbRepo::init().await;
    let app_data = get_app_data(repo, AuthService).await;

    let id_result = app_data.repo.insert_administrator(Username().fake::<String>(), SafeEmail().fake::<String>(), Password(5..10).fake::<String>()).await.unwrap();
    let admin = app_data.repo.query_administrator(id_result.id).await.unwrap().unwrap();

    assert!(admin.id == id_result.id);
}