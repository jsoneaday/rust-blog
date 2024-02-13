use sqlx::{Pool, Postgres, query_as, Error};
use crate::common::repository::mail::models::Mail;
use crate::common::repository::base::{ConnGetter, DbRepo};
use async_trait::async_trait;

mod internal {  
    use super::*;

    pub async fn query_latest_mail(conn: &Pool<Postgres>, page_size: i32, last_offset: i64) -> Result<Vec<Mail>, Error> {
        query_as::<_, Mail>("select * from mail order by updated_at desc limit $1 offset $2")
            .bind(page_size)
            .bind(last_offset)
            .fetch_all(conn)
            .await
    }

    pub async fn query_mail(conn: &Pool<Postgres>, mail_id: i64) -> Result<Option<Mail>, Error> {
        query_as::<_, Mail>("select * from mail where id = $1")
            .bind(mail_id)
            .fetch_optional(conn)
            .await
    }
}

#[async_trait]
pub trait QueryLatestMailFn {
    async fn query_latest_mail(&self, page_size: i32, last_offset: i64) -> Result<Vec<Mail>, Error>;
}

#[async_trait]
impl QueryLatestMailFn for DbRepo {
    async fn query_latest_mail(&self, page_size: i32, last_offset: i64) -> Result<Vec<Mail>, Error> {
        internal::query_latest_mail(self.get_conn(), page_size, last_offset).await
    }
}

#[async_trait]
pub trait QueryMailFn {
    async fn query_mail(&self, mail_id: i64) -> Result<Option<Mail>, Error>;
}

#[async_trait]
impl QueryMailFn for DbRepo {
    async fn query_mail(&self, mail_id: i64) -> Result<Option<Mail>, Error> {
        internal::query_mail(self.get_conn(), mail_id).await
    }
}