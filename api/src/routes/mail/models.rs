use actix_http::body::BoxBody;
use actix_web::{Responder, HttpResponse, http::header::ContentType};
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::common::repository::mail::models::Mail;

#[derive(Serialize, Debug, Clone)]
pub struct MailResponder {
    pub id: i64,
    pub updated_at: DateTime<Utc>,
    pub from: String,
    pub subject: String,
    pub message: String
}

impl Responder for MailResponder {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);
        
        match json_result {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(_) => HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("Failed to serialize MailResponder")
        }
    }
}

#[derive(Serialize, Debug)]
pub struct MailResponders(pub Vec<MailResponder>);

impl Responder for MailResponders {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let json_result = serde_json::to_string(&self);

        match json_result {
            Ok(body) => HttpResponse::Ok()
                .content_type(ContentType::json())
                .body(body),
            Err(_) => HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body("Failed to serialize MailResponders")
        }
    }
}

pub fn convert(mail: &Mail) -> MailResponder {
    MailResponder {
        id: mail.id,
        updated_at: mail.updated_at,
        from: mail.from.to_string(),
        subject: mail.subject.to_string(),
        message: mail.message.to_string(),        
    }
}