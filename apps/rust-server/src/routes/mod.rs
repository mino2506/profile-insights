use axum::{routing::{get, post}, Router};
use tower_http::trace::TraceLayer;

mod health;
mod hello;
mod echo;

pub fn router()-> Router {
  Router::new()
        .route("/health", get(health::handler))
        .route("/hello", get(hello::handler))
        .route("/echo", post(echo::handler))
        .layer(TraceLayer::new_for_http())
}