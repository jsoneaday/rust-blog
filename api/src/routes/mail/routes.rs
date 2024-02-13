use actix_web::web::{Data, Path};
use crate::{common::{authentication::auth_service::Authenticator, repository::{base::Repository, mail::repo::{QueryLatestMail, QueryMail}}}, routes::{app_state::AppState, base_model::PagingModel, stripped_down_error::StrippedDownError}};
use super::models::{convert, MailResponder, MailResponders};


pub async fn get_latest_mail<T: QueryLatestMail + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, path: Path<PagingModel>) -> Result<MailResponders, StrippedDownError> {
    let result = app_data.repo.query_latest_mail(path.page_size, path.last_offset).await;

    match result {
        Ok(mail) => {
            let post_responders = mail.iter().map(|post| convert(post)).collect::<Vec<MailResponder>>();
            Ok(MailResponders(post_responders))
        },
        Err(e) => Err(e.into())
    }
}

pub async fn get_mail<T: QueryMail + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, path: Path<i64>) -> Result<Option<MailResponder>, StrippedDownError> {
    let post_result = app_data.repo.query_mail(path.into_inner()).await;

    match post_result {
        Ok(opt_mail) => {
            match opt_mail {
                Some(mail) => Ok(Some(convert(&mail))),
                None => Ok(None)
            }            
        },
        Err(e) => Err(e.into())
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use chrono::Utc;
    use sqlx::Error;
    use crate::common::repository::mail::models::Mail;
    use jsonwebtoken::DecodingKey;
    use crate::{common::authentication::auth_service::{AuthService, AuthenticationError}, common_test::fixtures::get_app_data};
    use super::*;

    const FROM: &str = "dave@test.com";
    const SUBJECT: &str = "subject";
    const MESSAGE: &str = "Hello World! How are you?";
    struct MockDbRepo;
    struct MockAuthService;
    #[async_trait]
    impl Authenticator for MockAuthService {
        async fn is_authenticated(&self, _: String, _: Vec<(&str, &str)>, _: &DecodingKey) -> Result<bool, AuthenticationError> {
            Ok(true)
        }
    }

    #[async_trait]
    impl Repository for MockDbRepo {
        async fn init() -> Self {
            MockDbRepo
        }
    }

    #[async_trait]
    impl QueryLatestMail for MockDbRepo {
        async fn query_latest_mail(&self, _page_size: i32, _last_offset: i64) -> Result<Vec<Mail>, Error> {
            Ok(vec![
                Mail {
                    id: 1,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    from: FROM.to_string(),
                    subject: SUBJECT.to_string(),
                    message: MESSAGE.to_string()
                }
            ])
        }
    }

    #[async_trait]
    impl QueryMail for MockDbRepo {
        async fn query_mail(&self, _mail_id: i64) -> Result<Option<Mail>, Error> {
            Ok(Some(
                Mail {
                id: 1,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                from: FROM.to_string(),
                subject: SUBJECT.to_string(),
                message: MESSAGE.to_string()
            }))
        }
    }

    #[tokio::test]
    async fn test_get_latest_mail_returns_correctly() {
        let repo = MockDbRepo::init().await;
        let auth_service = AuthService;
        let app_data = get_app_data(repo, auth_service).await;        

        let mail = get_latest_mail(app_data, Path::from(PagingModel {
            page_size: 10,
            last_offset: 0
        })).await;

        assert!(mail.as_ref().is_ok());
        assert!(mail.as_ref().ok().unwrap().0.get(0).unwrap().from == FROM.to_string());
        assert!(mail.as_ref().ok().unwrap().0.get(0).unwrap().subject == SUBJECT.to_string());
        assert!(mail.as_ref().ok().unwrap().0.get(0).unwrap().message == MESSAGE.to_string());
    }

    #[tokio::test]
    async fn test_get_mail_returns_correctly() {
        let repo = MockDbRepo::init().await;
        let auth_service = AuthService;
        let app_data = get_app_data(repo, auth_service).await;        

        let mail_resp = get_mail(app_data, Path::from(1)).await;

        assert!(mail_resp.is_ok());
        match mail_resp {
            Ok(opt_mail) => match opt_mail {
                Some(mail) => {
                    assert!(mail.from == FROM.to_string());
                    assert!(mail.subject == SUBJECT.to_string());
                    assert!(mail.message == MESSAGE.to_string());
                },
                _ => panic!("Failed")
            },
            _ => panic!("Failed")
        }
    }
}