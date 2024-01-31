use actix_web::{web::{Json, Data, Path}, HttpRequest};
use log::error;
use crate::{
    routes::{base_model::{OutputId, PagingModel}, stripped_down_error::StrippedDownError, app_state::AppState, auth_helper::check_is_authenticated}, 
    common::{repository::{administrator::repo::QueryAdministratorFn, base::Repository, post::repo::{InsertPostFn, QueryPostsFn}}, authentication::auth_service::Authenticator}
};
use super::models::{NewPost, PostResponder, convert, PostResponders};

pub async fn create_post<T: InsertPostFn + QueryAdministratorFn + Repository, U: Authenticator>(app_data: Data<AppState<T, U>>, new_post: Json<NewPost>, req: HttpRequest) -> Result<OutputId, StrippedDownError> {
    let is_authenticated = check_is_authenticated(app_data.clone(), new_post.admin_id, req).await;
    if !is_authenticated {
        error!("Authentication Failed");
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