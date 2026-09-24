use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::application::error::ApplicationError;

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

pub enum ApiError {
    NotFound,
    BadRequest(String),
}

impl From<ApplicationError> for ApiError {
    fn from(error: ApplicationError) -> Self {
        match error {
            ApplicationError::JobNotFound => ApiError::NotFound,
            ApplicationError::InvalidRequest(message) => ApiError::BadRequest(message),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "job_not_found".to_string(),
                }),
            )
                .into_response(),
            ApiError::BadRequest(message) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { error: message }),
            )
                .into_response(),
        }
    }
}
