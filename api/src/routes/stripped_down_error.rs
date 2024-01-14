use actix_http::StatusCode;
use actix_web::{ResponseError, HttpResponse, http::header::ContentType};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error, PartialEq)]
pub enum StrippedDownError {
    #[display(fmt = "An internal error occurred. Please try again later.")]
    InternalError,
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
    #[display(fmt = "Authentication Failed. Email or password is incorrect.")]
    AuthenticationFailed,
    #[display(fmt = "Authorization for Resource Failed.")]
    AuthorizationFailed,
}

impl StrippedDownError {
    pub fn convert_to_stripped_error(e: sqlx::Error) -> StrippedDownError {
        match e {
            sqlx::Error::RowNotFound => StrippedDownError::InternalError,
            sqlx::Error::ColumnDecode { .. } => StrippedDownError::InternalError,
            sqlx::Error::Decode(_) => StrippedDownError::InternalError,
            sqlx::Error::PoolTimedOut => StrippedDownError::InternalError,
            sqlx::Error::PoolClosed => StrippedDownError::InternalError,
            sqlx::Error::WorkerCrashed => StrippedDownError::InternalError,
            #[cfg(feature = "migrate")]
            sqlx::Error::Migrate(_) => StrippedDownError::InternalError,
            _ => StrippedDownError::InternalError,
        }
    }
}

impl ResponseError for StrippedDownError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::plaintext())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            StrippedDownError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            StrippedDownError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            StrippedDownError::AuthenticationFailed => StatusCode::UNAUTHORIZED,
            StrippedDownError::AuthorizationFailed => StatusCode::UNAUTHORIZED
        }
    }
}

impl Into<StrippedDownError> for sqlx::Error {
    fn into(self) -> StrippedDownError {
        StrippedDownError::convert_to_stripped_error(self)
    }
}