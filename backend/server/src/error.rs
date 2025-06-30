use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("configuration error: {0}")]
    Config(#[from] anyhow::Error),
    #[error("database error: {0}")]
    Database(#[from] redis::RedisError),
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

#[derive(Serialize)]
struct ErrorResponse<'a> {
    message: &'a str,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::Network(_) => StatusCode::BAD_GATEWAY,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let msg = self.to_string();
        let body = Json(ErrorResponse { message: &msg });
        (status, body).into_response()
    }
}
