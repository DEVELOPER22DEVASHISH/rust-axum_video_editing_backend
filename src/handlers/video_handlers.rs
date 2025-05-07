use axum::{Json, response::IntoResponse};
use serde_json::json;

// #[tokio::main]
pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok"
    }))
}
