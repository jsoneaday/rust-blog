use actix_web::{web::Data, HttpRequest, HttpResponse};
use log::{info, error};
use crate::{common::{repository::{administrator::repo::QueryAdministratorFn, base::Repository}, authentication::auth_service::Authenticator}, routes::route_utils::get_header_strings};
use super::{app_state::AppState, authentication::models::LoginResponse};

pub async fn check_is_authenticated<T: QueryAdministratorFn + Repository, U: Authenticator>(
    app_data: Data<AppState<T, U>>,
    admin_id: i64,
    req: HttpRequest
) -> bool {
    let admin = app_data.repo.query_administrator(admin_id).await.unwrap().unwrap();
    
    let headers = get_header_strings(req.headers());
    info!("headers {:?}", headers);
    let is_authenticated_result = app_data.auth_service.is_authenticated(admin.user_name, headers, &app_data.auth_keys.decoding_key).await;
    match is_authenticated_result {
        Ok(result) => match result {
            true => {
                info!("Successfully authorized");
                true
            },
            false => {
                info!("Failed authorization");
                false
            }
        },
        Err(_) => {
            error!("Authorization attempt failed");
            false
        }
    }
}

pub async fn get_access_token_from_login_resp_httpresponse(httpresponse: HttpResponse) -> String {
    let (_res, body) = httpresponse.into_parts();
    let bytes = actix_http::body::to_bytes(body).await.unwrap();
    let resp_body = String::from_utf8_lossy(&bytes);
    let login_resp: Result<LoginResponse, serde_json::Error> = serde_json::from_str(&resp_body);
    login_resp.unwrap().access_token.to_string()
}

pub async fn get_access_token_from_str_body_httpresponse(httpresponse: HttpResponse) -> String {    
    let (_res, body) = httpresponse.into_parts();
    let bytes = actix_http::body::to_bytes(body).await.unwrap();
    let resp_body = String::from_utf8_lossy(&bytes);
    resp_body.to_string()
}