use axum::{Router, routing::get};
use sea_orm::DatabaseConnection;
use crate::handlers::video_handlers;

pub fn create_video_routes(db: DatabaseConnection) -> Router {
    Router::new()
        .route("/health", get(video_handlers::health_check))
        .with_state(db)
}
