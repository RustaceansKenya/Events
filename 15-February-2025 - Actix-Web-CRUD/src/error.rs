use actix_web::{body, http::StatusCode, HttpResponse, ResponseError};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0:?}")]
    ValidationError(String),
    #[error("User not found")]
    NotFound,
    #[error("Internal Server Error")]
    InternalError(String),
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::ValidationError(_) => StatusCode::BAD_REQUEST,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<body::BoxBody> {
        let mut response = HttpResponse::build(self.status_code());

        match self {
            Self::ValidationError(msg) => response.json(json!({
                "msg": msg,
                "success": false
            })),
            Self::NotFound => response.json(json!({
                "msg": "User not found",
                "success": false
            })),
            Self::InternalError(_) => response.json(json!({
                "msg": "Internal Server Error",
                "success": false
            })),
        }
    }
}
