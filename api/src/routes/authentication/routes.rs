use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    web::{Data, Json}, 
    HttpResponse,
    http::header::ContentType, HttpRequest
};
use chrono::Utc;
use log::{error, info};
use crate::{
    routes::app_state::AppState, 
    common::{
        repository::{
            base::Repository, 
            administrator::{repo::{AuthenticateDbFn, QueryAdministratorFn}, models::AuthenticateResult}
        }, 
        authentication::auth_service::{get_token, STANDARD_REFRESH_TOKEN_EXPIRATION, Authenticator, STANDARD_ACCESS_TOKEN_EXPIRATION, REFRESH_TOKEN_LABEL, decode_token}
    }
};
use super::models::{LoginCredential, RefreshToken};


pub async fn refresh_access_token<T: Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, json: Json<RefreshToken>, req: HttpRequest) -> HttpResponse {    
    let refresh_cookie = req.cookie(REFRESH_TOKEN_LABEL);

    match refresh_cookie {
        Some(cookie) => {
            let cookie_val = cookie.value();            
            let refresh_token = decode_token(cookie_val, &app_data.auth_keys.decoding_key);
            let refresh_user_name = refresh_token.sub;
            let current_access_token = decode_token(&json.old_token, &app_data.auth_keys.decoding_key);
            if refresh_user_name == current_access_token.sub && refresh_token.exp >= (Utc::now().timestamp() as usize) {
                let new_access_token = get_token(refresh_user_name, &app_data.auth_keys.encoding_key, Some(STANDARD_ACCESS_TOKEN_EXPIRATION));
                info!("Access token {}", new_access_token);
                return HttpResponse::Ok()
                    .body(new_access_token);
            } else {
                info!("Refresh access token failed");
                return HttpResponse::BadRequest()
                    .content_type(ContentType::json())
                    .body("Refresh access token failed. Your request token is expired");
            }            
        },
        None => {
            error!("No refresh cookie found");
            return HttpResponse::BadRequest()
                .content_type(ContentType::json())
                .body("Authentication failed. Your request is missing the refresh token")
        }
    };
}


pub async fn login<T: AuthenticateDbFn + QueryAdministratorFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, json: Json<LoginCredential>) 
    -> HttpResponse {
    let auth_result = app_data.repo.authenticate_db(json.email.clone(), json.password.clone()).await;
    
    match auth_result {
        Ok(result) => {
            match result {
                AuthenticateResult::Success { id } => {
                    #[allow(unused)] let mut user_name = "".to_string();                    
                    #[allow(unused)] let mut http_response: Option<HttpResponse> = None;
                    
                    let user = app_data.repo.query_administrator(id).await;
                    match user {
                        Ok(opt_user) => {
                            if let Some(usr) = opt_user {
                                user_name = usr.user_name;
                                let (refresh_cookie, access_token) = get_refresh_and_access_token_response(app_data, user_name.as_str());
                                http_response = Some(HttpResponse::Ok()
                                    .cookie(refresh_cookie)
                                    .body(access_token));
                            } else {
                                error!("Authentication failed. Developer not found");
                                http_response = Some(HttpResponse::Unauthorized()
                                    .content_type(ContentType::json())
                                    .body("Authentication failed. Developer not found"));
                            }
                        },
                        Err(_) => {
                            error!("Authentication failed. Error occurred while trying to get developer");
                            http_response = Some(HttpResponse::Unauthorized()
                                .content_type(ContentType::json())
                                .body("Authentication failed. Error occurred while trying to get developer"));
                        }
                    }

                    return http_response.unwrap();          
                },
                _ => {
                    HttpResponse::Unauthorized()
                        .content_type(ContentType::json())
                        .body("Authentication failed. Wrong email or password")
                }
            }
        }
        Err(_) => {
            error!("Authentication failed. Server error");
            HttpResponse::Unauthorized()
                .content_type(ContentType::json())
                .body("Authentication failed. Server error occurred while trying to authenticate")
        }
    }  
}

fn get_refresh_and_access_token_response<'a, T: AuthenticateDbFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, user_name: &'a str) -> (Cookie<'a>, String) {
    let access_token = get_token(user_name.to_string(), &app_data.auth_keys.encoding_key, Some(STANDARD_ACCESS_TOKEN_EXPIRATION));
    let refresh_token = get_token(user_name.to_string(), &app_data.auth_keys.encoding_key, None);
    let refresh_cookie = Cookie::build(REFRESH_TOKEN_LABEL, refresh_token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(STANDARD_REFRESH_TOKEN_EXPIRATION, 0))
        .http_only(true)
        .secure(false)
        //.same_site(SameSite::Lax)
        .finish();
                    
    (refresh_cookie, access_token)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_http::{StatusCode, body};
    use async_trait::async_trait;
    use fake::{faker::internet::en::{FreeEmail, Password}, Fake};
    use jsonwebtoken::DecodingKey;
    use crate::{
        common::{
            repository::administrator::{repo::AuthenticateDbFn, models::Administrator}, 
            authentication::auth_service::{STANDARD_REFRESH_TOKEN_EXPIRATION, AuthenticationError}
        }, 
        common_test::fixtures::get_app_data
    };

    const USERNAME: &str = "tester";
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
    impl AuthenticateDbFn for MockDbRepo {
        async fn authenticate_db(&self, _: String, _: String) -> Result<AuthenticateResult, sqlx::Error> {
            Ok(AuthenticateResult::Success{ id: 1 })
        }
    }

    #[async_trait]
    impl QueryAdministratorFn for MockDbRepo {
        async fn query_administrator(&self, _id: i64) -> Result<Option<Administrator>, sqlx::Error> {
            Ok(Some(Administrator {
                id: 0,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                user_name: USERNAME.to_string(),
                email: FreeEmail().fake::<String>(),
                password: "123".to_string()
            }))
        }
    }

    #[tokio::test]
    async fn test_login_routes_httpresponse() {
        let repo = MockDbRepo::init().await;
        let auth_service = MockAuthService;
        let app_data = get_app_data(repo, auth_service).await;

        let result = login(app_data.clone(), Json(LoginCredential { email: FreeEmail().fake::<String>(), password: Password(5..10).fake::<String>() })).await;
        assert!(result.status() == StatusCode::OK);

        let (res, mut body) = result.into_parts();
        let bytes = body::to_bytes(&mut body).await.ok().unwrap();
        let token_str = String::from_utf8_lossy(&bytes);
        let token = decode_token(&token_str, &app_data.auth_keys.decoding_key);
        assert!(token.exp >= STANDARD_ACCESS_TOKEN_EXPIRATION as usize);
        assert!(token.sub == USERNAME.to_string());

        let cookie = res.cookies().last().unwrap();
        let refresh_token = cookie.value();
        let claims = decode_token(refresh_token, &app_data.auth_keys.decoding_key);
        
        assert!(claims.exp >= STANDARD_REFRESH_TOKEN_EXPIRATION as usize);
        assert!(claims.sub == USERNAME.to_string());        
    }
}