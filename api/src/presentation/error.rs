use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::application::error::ApplicationError;

#[derive(Serialize)]
struct ErrorResponse {
    error: &'static str,
}

pub enum ApiError {
    NotFound,
}

impl From<ApplicationError> for ApiError {
    fn from(error: ApplicationError) -> Self {
        match error {
            ApplicationError::JobNotFound => ApiError::NotFound,
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse {
                    error: "job_not_found",
                }),
            )
                .into_response(),
        }
    }
}
