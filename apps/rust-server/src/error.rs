use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Serialize)]
struct ErrorBody {
    message: String,
}

#[derive(Debug)]
pub enum AppError {
    BadRequest(String),
    Internal(String),
}

impl std::error::Error for AppError {}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AppError::BadRequest(m) => write!(f, "{}", m),
            AppError::Internal(m) => write!(f, "{}", m),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // ここだけ HTTP 用の処理
        let (status, message) = match self {
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, m),
            AppError::Internal(m) => (StatusCode::INTERNAL_SERVER_ERROR, m),
        };

        let body = Json(ErrorBody { message });

        (status, body).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
