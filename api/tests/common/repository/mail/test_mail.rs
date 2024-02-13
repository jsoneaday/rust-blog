use rustyindie_api::{
    common::{repository::{base::{DbRepo, Repository}, mail::repo::{QueryMailFn, QueryLatestMailFn}}, authentication::auth_service::AuthService}, 
    common_test::fixtures::get_app_data
};

#[tokio::test]
async fn test_query_latest_mail_return_correct_mail() {
    let repo = DbRepo::init().await;
    let app_data = get_app_data(repo, AuthService).await;

    // requires the db test setup scripts to run
    let mail_result = app_data.repo.query_latest_mail(10, 0).await.unwrap();
    
    assert!(mail_result.len() > 0);
    assert!(mail_result.first().unwrap().updated_at > mail_result.last().unwrap().updated_at);
}

#[tokio::test]
async fn test_query_mail_return_correct_mail() {
    let repo = DbRepo::init().await;
    let app_data = get_app_data(repo, AuthService).await;

    // requires the db test setup scripts to run
    let mail_result = app_data.repo.query_mail(1).await.unwrap();
    
    assert!(mail_result.is_some());
}