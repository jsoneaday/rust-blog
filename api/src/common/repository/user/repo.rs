use sqlx::{Postgres, query_as, Pool, Error};
use crate::common::repository::{user::models::AuthenticateResult, base::{EntityId, DbRepo, ConnGetter}};
use async_trait::async_trait;
use crate::common::repository::user::models::User;

mod internal {   
    use super::*;    

    pub async fn authenticate_db(conn: &Pool<Postgres>, email: String, password: String) -> Result<AuthenticateResult, sqlx::Error> {        
        let result = query_as::<_, EntityId>("select id from user where email = $1 and password = $2")
            .bind(email)
            .bind(password)
            .fetch_optional(conn)
            .await;

        match result {
            Ok(opt_entity) => match opt_entity {
                Some(entity) => Ok(AuthenticateResult::Success { id: entity.id }),
                None => Ok(AuthenticateResult::Failure)
            },
            Err(e) => Err(e.into())
        }
    }

    pub async fn query_user(conn: &Pool<Postgres>, id: i64) -> Result<Option<User>, Error> {
        query_as::<_, User>("select * from User where id = $1")
            .bind(id)
            .fetch_optional(conn)
            .await
    }
}

#[async_trait]
pub trait AuthenticateDbFn {
    async fn authenticate_db(&self, email: String, password: String) -> Result<AuthenticateResult, sqlx::Error>;
}

#[async_trait]
impl AuthenticateDbFn for DbRepo {
    async fn authenticate_db(&self, email: String, password: String) -> Result<AuthenticateResult, sqlx::Error> {
        internal::authenticate_db(self.get_conn(), email, password).await
    }
}

#[async_trait]
pub trait QueryUserFn {
    async fn query_user(&self, id: i64) -> Result<Option<User>, Error>;
}

#[async_trait]
impl QueryUserFn for DbRepo {
    async fn query_user(&self, id: i64) -> Result<Option<User>, Error> {
        internal::query_user(self.get_conn(), id).await
    }
}