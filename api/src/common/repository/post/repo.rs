use async_trait::async_trait;
use sqlx::{Postgres, Pool, query_as, Error};
use crate::common::repository::{post::models::Post, base::{DbRepo, ConnGetter}};

mod internal {
    use super::*;

    pub async fn query_posts(conn: &Pool<Postgres>, page_size: i32, last_offset: i64) -> Result<Vec<Post>, Error> {
        query_as::<_, Post>("select * from post where limit $1 offset $2")
            .bind(page_size)
            .bind(last_offset)
            .fetch_all(conn)
            .await
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