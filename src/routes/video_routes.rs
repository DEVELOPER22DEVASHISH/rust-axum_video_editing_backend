use axum::{Router, routing::{get, post}};
use std::sync::Arc;
use crate::services::video_service::VideoService;
use crate::handlers::video_handlers;
use sea_orm::DatabaseConnection;

pub fn create_video_routes(db: DatabaseConnection) -> Router {
    let video_service = Arc::new(VideoService::new(db));

    Router::new()
        .route("/health", get(video_handlers::health_check))
        .route("/videoupload", post(video_handlers::upload_video_handler))
        .route("/video/{id}/trim", post(video_handlers::trim_video_handler))
        .route("/video/{id}/subtitles", post(video_handlers::add_subtitles_handler))
        .route("/video/{id}/render", post(video_handlers::render_video_handler))
        .route("/video/{id}/download", get(video_handlers::download_video_handler))
        .with_state(video_service)
}

