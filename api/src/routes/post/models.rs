use actix_http::body::BoxBody;
use actix_web::{Responder, HttpResponse, http::header::ContentType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::common::repository::post::models::Post;

#[derive(Deserialize)]
pub struct DeletePost {
    pub post_id: i64,
    pub admin_id: i64
}

#[derive(Deserialize)]
pub struct NewPost {
    pub title: String,
    pub message: String,
    pub admin_id: i64
}

#[derive(Serialize, Debug)]
pub struct PostResponder {
    pub id: i64,
    pub updated_at: DateTime<Utc>,
    pub title: String,
    pub message: String,
    pub admin_id: i64
}

impl Responder for PostResponder {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);
        
        match json_result {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(_) => HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("Failed to serialize PostResponder")
        }
    }
}

#[derive(Serialize, Debug)]
pub struct PostResponders(pub Vec<PostResponder>);

impl Responder for PostResponders {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);

        match json_result {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(_) => HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("Failed to serialize PostResponders")
        }
    }
}

pub fn convert(post: &Post) -> PostResponder {
    PostResponder {
        id: post.id,
        updated_at: post.updated_at,
        title: post.title.to_string(),
        message: post.message.to_string(),
        admin_id: post.admin_id
    }
}