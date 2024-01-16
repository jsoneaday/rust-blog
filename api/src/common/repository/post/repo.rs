use async_trait::async_trait;
use sqlx::{Postgres, Pool, query_as, Error};
use crate::common::repository::{post::models::Post, base::{DbRepo, ConnGetter, EntityId}};

mod internal {
    use super::*;

    pub async fn insert_post(conn: &Pool<Postgres>, message: String, admin_id: i64) -> Result<EntityId, Error> {
        query_as::<_, EntityId>("insert into post (message, admin_id) values ($1, $2) returning id")
            .bind(message)
            .bind(admin_id)
            .fetch_one(conn)
            .await
    }

    pub async fn query_posts(conn: &Pool<Postgres>, page_size: i32, last_offset: i64) -> Result<Vec<Post>, Error> {
        query_as::<_, Post>("select * from post order by updated_at desc limit $1 offset $2")
            .bind(page_size)
            .bind(last_offset)
            .fetch_all(conn)
            .await
    }
}

#[async_trait]
pub trait InsertPostFn {
    async fn insert_post(&self, message: String, admin_id: i64) -> Result<EntityId, Error>;
}

#[async_trait]
impl InsertPostFn for DbRepo {
    async fn insert_post(&self, message: String, admin_id: i64) -> Result<EntityId, Error> {
        internal::insert_post(self.get_conn(), message, admin_id).await
    }
}

#[async_trait]
pub trait QueryPostsFn {
    async fn query_posts(&self, page_size: i32, last_offset: i64) -> Result<Vec<Post>, Error>;
}

#[async_trait]
impl QueryPostsFn for DbRepo {
    async fn query_posts(&self, page_size: i32, last_offset: i64) -> Result<Vec<Post>, Error> {
        internal::query_posts(self.get_conn(), page_size, last_offset).await
    }
}