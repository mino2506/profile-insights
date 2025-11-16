use crate::error::{AppError, AppResult};
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct EchoPayload {
    text: String,
}

pub async fn handler(Json(payload): Json<EchoPayload>) -> AppResult<Json<EchoPayload>> {
    if payload.text.trim().is_empty() {
        return Err(AppError::BadRequest("text must not be empty".into()));
    }

    Ok(Json(payload))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::Json;

    #[tokio::test]
    async fn echo_returns_same_payload() {
        let req = EchoPayload {
            text: "Hello, World!".into(),
        };

        let res = handler(Json(req)).await.expect("handler should not fail");

        let Json(res) = res;

        assert_eq!(res.text, "Hello, World!".to_string());
    }

    #[tokio::test]
    async fn empty_text_returns_bad_request() {
        let req = EchoPayload {
            text: "   ".into(), // trim すると空
        };

        let err = handler(Json(req))
            .await
            .expect_err("should fail on empty text");

        // バリアントまでチェックしたければ match / matches! を使う
        match err {
            AppError::BadRequest(msg) => {
                assert_eq!(msg, "text must not be empty");
            }
            other => panic!("expected BadRequest, got: {:?}", other),
        }
    }
}
