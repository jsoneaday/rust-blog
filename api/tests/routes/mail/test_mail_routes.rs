use rustyindie_api::{
    common::{authentication::auth_service::AuthService, repository::base::{DbRepo, Repository}}, 
    common_test::fixtures::get_app_data, 
    routes::{
        base_model::PagingModel, 
        mail::routes::{get_latest_mail, get_mail}
    }
    
};
use actix_web::web::Path;

#[tokio::test]
async fn test_get_latest_mail_route_returns_atleast_two_mail() {
    let repo = DbRepo::init().await;
    let auth_service = AuthService;
    let app_data = get_app_data(repo, auth_service).await;

    // mails created in db setup code already
    let mail_res = get_latest_mail(app_data, Path::from(PagingModel {
        page_size: 10,
        last_offset: 0
    }))
    .await;

    assert!(mail_res.is_ok());
    assert!(mail_res.unwrap().0.len() > 1);
}

#[tokio::test]
async fn test_get_mail_route_returns_mail() {
    let repo = DbRepo::init().await;
    let auth_service = AuthService;
    let app_data = get_app_data(repo, auth_service).await;

    // mails created in db setup code already
    let mail_res = get_mail(app_data, Path::from(1)).await;

    assert!(mail_res.is_ok());
    assert!(mail_res.ok().is_some());
}