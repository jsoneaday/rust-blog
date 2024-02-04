use actix_web::{web::{Json, Data, Path}, HttpRequest, HttpResponse};
use log::error;
use crate::{
    routes::{base_model::{OutputId, PagingModel}, stripped_down_error::StrippedDownError, app_state::AppState, auth_helper::check_is_authenticated}, 
    common::{
        repository::{administrator::repo::QueryAdministratorFn, base::Repository, post::repo::{DeletePostFn, InsertPostFn, QueryPostFn, QueryPostsFn, QueryPostsPreviewFn, UpdatePostFn}}, 
        authentication::auth_service::Authenticator
    }
};
use super::models::{convert, DeletePost, UpdatePost, NewPost, PostResponder, PostResponders};

pub async fn create_post<T: InsertPostFn + QueryAdministratorFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, new_post: Json<NewPost>, req: HttpRequest) -> Result<OutputId, StrippedDownError> {
    let is_authenticated = check_is_authenticated(app_data.clone(), new_post.admin_id, req).await;
    if !is_authenticated {
        error!("create_post error: Authentication Failed");
        return Err(StrippedDownError::AuthenticationFailed);
    }

    let entity_result = app_data.repo.insert_post(new_post.title.clone(), new_post.message.clone(), new_post.admin_id).await;

    match entity_result {
        Ok(entity) => Ok(OutputId { id: entity.id }),
        Err(e) => Err(e.into())
    }
}

pub async fn get_posts<T: QueryPostsFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, path: Path<PagingModel>) -> Result<PostResponders, StrippedDownError> {
    let posts_result = app_data.repo.query_posts(path.page_size, path.last_offset).await;

    match posts_result {
        Ok(posts) => {
            let post_responders = posts.iter().map(|post| convert(post)).collect::<Vec<PostResponder>>();
            Ok(PostResponders(post_responders))
        },
        Err(e) => Err(e.into())
    }
}

pub async fn get_post<T: QueryPostFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, path: Path<i64>) -> Result<Option<PostResponder>, StrippedDownError> {
    let post_result = app_data.repo.query_post(path.into_inner()).await;

    match post_result {
        Ok(opt_post) => {
            match opt_post {
                Some(post) => Ok(Some(PostResponder {
                    id: post.id,
                    updated_at: post.updated_at,
                    title: post.title,
                    message: post.message,
                    admin_id: post.admin_id
                })),
                None => Ok(None)
            }
            
        },
        Err(e) => Err(e.into())
    }
}

pub async fn get_post_previews<T: QueryPostsPreviewFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, path: Path<PagingModel>) -> Result<PostResponders, StrippedDownError> {
    let posts_result = app_data.repo.query_post_previews(path.page_size, path.last_offset).await;

    match posts_result {
        Ok(posts) => {
            let post_responders = posts.iter().map(|post| convert(post)).collect::<Vec<PostResponder>>();
            Ok(PostResponders(post_responders))
        },
        Err(e) => Err(e.into())
    }
}

pub async fn delete_post<T: DeletePostFn + QueryAdministratorFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, json: Json<DeletePost>, req: HttpRequest) 
    -> HttpResponse {
    let is_authenticated = check_is_authenticated(app_data.clone(), json.admin_id, req).await;
    if !is_authenticated {
        error!("delete_post error: Authentication Failed");
        return HttpResponse::Unauthorized().body("Failed to authorize for deletion");
    }

    let result = app_data.repo.delete_post(json.post_id).await;

    match result {
        Ok(_) => HttpResponse::NoContent().into(),
        Err(_) => HttpResponse::InternalServerError().body("Failed to delete")
    }
}

pub async fn update_post<T: UpdatePostFn + QueryAdministratorFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, json: Json<UpdatePost>, req: HttpRequest) 
    -> HttpResponse {
    let is_authenticated = check_is_authenticated(app_data.clone(), json.admin_id, req).await;
    if !is_authenticated {
        error!("update_post error: Authentication Failed");
        return HttpResponse::Unauthorized().body("Failed to authorize for update");
    }

    let result = app_data.repo.update_post(json.post_id, json.title.clone(), json.message.clone()).await;

    match result {
        Ok(_) => HttpResponse::NoContent().into(),
        Err(_) => HttpResponse::InternalServerError().body("Failed to update")
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;
    use chrono::Utc;
    use fake::{faker::lorem::en::Sentence, Fake};
    use sqlx::Error;
    use crate::{
        common::{
            authentication::auth_service::STANDARD_ACCESS_TOKEN_EXPIRATION, 
            repository::{administrator::models::Administrator, base::EntityId, post::{models::Post, repo::InsertPostFn}}
        }, 
        common_test::fixtures::get_fake_httprequest_with_bearer_token
    };
    use jsonwebtoken::DecodingKey;
    use crate::{common::authentication::auth_service::{AuthService, AuthenticationError}, common_test::fixtures::get_app_data};
    use super::*;

    const MOCK_ENTITY_ID: i64 = 10;
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
    impl QueryPostsFn for MockDbRepo {
        async fn query_posts(&self, _page_size: i32, _last_offset: i64) -> Result<Vec<Post>, Error> {
            Ok(vec![Post {
                id: 1,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                title: "title".to_string(),
                message: "message".to_string(),
                admin_id: 1
            }])
        }
    }

    #[async_trait]
    impl QueryPostsPreviewFn for MockDbRepo {
        async fn query_post_previews(&self, _page_size: i32, _last_offset: i64) -> Result<Vec<Post>, Error> {
            Ok(vec![Post {
                id: 1,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                title: "title".to_string(),
                message: "message".to_string(),
                admin_id: 1
            }])
        }
    }

    #[async_trait]
    impl QueryPostFn for MockDbRepo {
        async fn query_post(&self, _id: i64) -> Result<Option<Post>, Error> {
            Ok(Some(Post {
                id: MOCK_ENTITY_ID,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                title: "title".to_string(),
                message: "message".to_string(),
                admin_id: 1
            }))
        }
    }

    #[async_trait]
    impl DeletePostFn for MockDbRepo {
        async fn delete_post(&self, _id: i64) -> Result<(), Error> {
            Ok(())
        }
    }

    #[async_trait]
    impl UpdatePostFn for MockDbRepo {
        async fn update_post(&self, _id: i64, _title: String, _message: String) -> Result<(), Error> {
            Ok(())
        }
    }

    #[async_trait]
    impl InsertPostFn for MockDbRepo {
        async fn insert_post(&self, _title: String, _message: String, _admin_id: i64) -> Result<EntityId, Error> {
            Ok(EntityId { id: MOCK_ENTITY_ID })
        }
    }

    #[async_trait]
    impl QueryAdministratorFn for MockDbRepo {
        async fn query_administrator(&self, _id: i64) -> Result<Option<Administrator>, Error> {
            Ok(Some(Administrator {
                id: 1,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                user_name: "dave".to_string(),
                email: "test@test.com".to_string(),
                password: "123".to_string()
            }))
        }
    }

    #[tokio::test]
    async fn test_create_post_returns_post_successfully() {
        let repo = MockDbRepo::init().await;
        let auth_service = AuthService;
        let app_data = get_app_data(repo, auth_service).await;
        let user_name = "dave".to_string();
        let title = "title".to_string();
        let message = Sentence(1..2).fake::<String>();

        let req = get_fake_httprequest_with_bearer_token(user_name, &app_data.auth_keys.encoding_key, "/v1/post", 1, Some(STANDARD_ACCESS_TOKEN_EXPIRATION));

        let created_post = create_post(app_data, Json(NewPost {
            title,
            message,
            admin_id: 1
        }), req).await;

        println!("created_post {:?}", created_post.as_ref().err());

        assert!(created_post.as_ref().is_ok());
        assert!(created_post.unwrap().id == MOCK_ENTITY_ID);
    }

    #[tokio::test]
    async fn test_get_posts_returns_post_successfully() {
        let repo = MockDbRepo::init().await;
        let auth_service = AuthService;
        let app_data = get_app_data(repo, auth_service).await;
        let user_name = "dave".to_string();
        let title = "title".to_string();
        let message = Sentence(1..2).fake::<String>();

        let req = get_fake_httprequest_with_bearer_token(user_name, &app_data.auth_keys.encoding_key, "/v1/post", 1, Some(STANDARD_ACCESS_TOKEN_EXPIRATION));

        _ = create_post(app_data.clone(), Json(NewPost {
            title,
            message,
            admin_id: 1
        }), req).await;

        let posts = get_posts(app_data, Path::from(PagingModel {
            page_size: 10,
            last_offset: 0
        })).await;

        assert!(posts.is_ok());
        assert!(posts.unwrap().0.len() > 0);
    }

    #[tokio::test]
    async fn test_get_post_previews_returns_post_successfully() {
        let repo = MockDbRepo::init().await;
        let auth_service = AuthService;
        let app_data = get_app_data(repo, auth_service).await;
        let user_name = "dave".to_string();
        let title = "title".to_string();
        let message = Sentence(1..2).fake::<String>();

        let req = get_fake_httprequest_with_bearer_token(user_name, &app_data.auth_keys.encoding_key, "/v1/post", 1, Some(STANDARD_ACCESS_TOKEN_EXPIRATION));

        _ = create_post(app_data.clone(), Json(NewPost {
            title,
            message,
            admin_id: 1
        }), req).await;

        let posts = get_post_previews(app_data, Path::from(PagingModel {
            page_size: 10,
            last_offset: 0
        })).await;

        assert!(posts.is_ok());
        assert!(posts.unwrap().0.len() > 0);
    }

    #[tokio::test]
    async fn test_get_post_returns_post_successfully() {
        let repo = MockDbRepo::init().await;
        let auth_service = AuthService;
        let app_data = get_app_data(repo, auth_service).await;
        let user_name = "dave".to_string();
        let title = "title".to_string();
        let message = Sentence(1..2).fake::<String>();

        let req = get_fake_httprequest_with_bearer_token(user_name, &app_data.auth_keys.encoding_key, "/v1/post", 1, Some(STANDARD_ACCESS_TOKEN_EXPIRATION));

        let created_post = create_post(app_data.clone(), Json(NewPost {
            title,
            message,
            admin_id: 1
        }), req).await;
        let created_post_id = created_post.unwrap().id;

        let post_resp = get_post(app_data, Path::from(created_post_id)).await;

        match post_resp {
            Ok(post_opt) => {
                match post_opt {
                    Some(post) => {
                        println!("post: {:?}, created id: {}", post, created_post_id);
                        assert!(post.id == created_post_id);        
                    },
                    None => panic!("failed None")
                }
            },
            Err(_) => panic!("failed error")
        }        
    }

    #[tokio::test]
    async fn test_delete_post_deletes_post_successfully() {
        let repo = MockDbRepo::init().await;
        let auth_service = AuthService;
        let app_data = get_app_data(repo, auth_service).await;
        let user_name = "dave".to_string();
        let title = "title".to_string();
        let message = Sentence(1..2).fake::<String>();

        let req = get_fake_httprequest_with_bearer_token(user_name, &app_data.auth_keys.encoding_key, "/v1/post", 1, Some(STANDARD_ACCESS_TOKEN_EXPIRATION));

        let created_post = create_post(app_data.clone(), Json(NewPost {
            title,
            message,
            admin_id: 1
        }), req.clone()).await;
        let created_post_id = created_post.unwrap().id;

        let post_resp = delete_post(app_data, Json(DeletePost {
            post_id: created_post_id,
            admin_id: 1
        }), req).await;

        assert!(post_resp.error().is_none());
    }

    #[tokio::test]
    async fn test_update_post_updates_post_values_correctly() {
        let repo = MockDbRepo::init().await;
        let auth_service = AuthService;
        let app_data = get_app_data(repo, auth_service).await;
        let user_name = "dave".to_string();
        let start_title = Sentence(1..2).fake::<String>();
        let start_message = Sentence(2..4).fake::<String>();
        let update_title = Sentence(1..2).fake::<String>();
        let update_message = Sentence(3..4).fake::<String>();

        let req = get_fake_httprequest_with_bearer_token(user_name, &app_data.auth_keys.encoding_key, "/v1/update_post", 1, Some(STANDARD_ACCESS_TOKEN_EXPIRATION));

        let created_post = create_post(app_data.clone(), Json(NewPost {
            title: start_title,
            message: start_message,
            admin_id: 1
        }), req.clone()).await;
        let created_post_id = created_post.unwrap().id;

        let post_resp = update_post(app_data, Json(UpdatePost {
            post_id: created_post_id,
            admin_id: 1,
            title: update_title,
            message: update_message
        }), req).await;

        assert!(post_resp.error().is_none());
    }
}