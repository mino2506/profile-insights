use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct HelloResponse {
    message: String,
}

pub async fn handler() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "hello from axum".into(),
    })
}
