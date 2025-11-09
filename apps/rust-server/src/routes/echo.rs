use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct EchoPayload {
    text: String,
}

pub async fn handler(Json(payload): Json<EchoPayload>) -> Json<EchoPayload> {
    Json(payload)
}