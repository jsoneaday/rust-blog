use sqlx::{Postgres, query_as, Pool, Error};
use crate::common::repository::{administrator::models::AuthenticateResult, base::{EntityId, DbRepo, ConnGetter}};
use async_trait::async_trait;
use crate::common::repository::administrator::models::Administrator;

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

    pub async fn query_administrator(conn: &Pool<Postgres>, id: i64) -> Result<Option<Administrator>, Error> {
        query_as::<_, Administrator>("select * from User where id = $1")
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
pub trait QueryAdministratorFn {
    async fn query_administrator(&self, id: i64) -> Result<Option<Administrator>, Error>;
}

#[async_trait]
impl QueryAdministratorFn for DbRepo {
    async fn query_administrator(&self, id: i64) -> Result<Option<Administrator>, Error> {
        internal::query_administrator(self.get_conn(), id).await
    }
}