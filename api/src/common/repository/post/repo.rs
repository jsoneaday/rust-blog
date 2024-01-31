use async_trait::async_trait;
use sqlx::{Postgres, Pool, query_as, Error};
use crate::common::repository::{post::models::Post, base::{DbRepo, ConnGetter, EntityId}};

mod internal {
    use super::*;

    pub async fn insert_post(conn: &Pool<Postgres>, title: String, message: String, admin_id: i64) -> Result<EntityId, Error> {
        query_as::<_, EntityId>("insert into post (title, message, admin_id) values ($1, $2, $3) returning id")
            .bind(title)
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
    async fn insert_post(&self, title: String, message: String, admin_id: i64) -> Result<EntityId, Error>;
}

#[async_trait]
impl InsertPostFn for DbRepo {
    async fn insert_post(&self, title: String, message: String, admin_id: i64) -> Result<EntityId, Error> {
        internal::insert_post(self.get_conn(), title, message, admin_id).await
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

#[async_trait]
pub trait QueryPostsPreviewFn {
    async fn query_post_previews(&self, page_size: i32, last_offset: i64) -> Result<Vec<Post>, Error>;
}

#[async_trait]
impl QueryPostsPreviewFn for DbRepo {
    async fn query_post_previews(&self, page_size: i32, last_offset: i64) -> Result<Vec<Post>, Error> {
        let post_result = internal::query_posts(self.get_conn(), page_size, last_offset).await;
        match post_result {
            Ok(posts) => Ok(posts.iter().map(|post| Post {
                id: post.id,
                created_at: post.created_at,
                updated_at: post.updated_at,
                title: post.title.to_string(),
                message: post.message[0..if post.message.len() < 250 { post.message.len() } else { 250 }].to_string(),
                admin_id: post.admin_id
            }).collect::<Vec<Post>>()),
            Err(e) => Err(e)
        }
    }
}