use axum::{Router, routing::{get, post}};
use std::sync::Arc;
use crate::services::video_service::VideoService;
use crate::handlers::video_handlers;
use sea_orm::DatabaseConnection;

pub fn create_video_routes(db: DatabaseConnection) -> Router {
    // Create a shared (Arc) instance of your VideoService
    let video_service = Arc::new(VideoService::new(db));

    // Define all your routes and attach the service as state
    Router::new()
        // Health check (optional, but recommended)
        .route("/health", get(video_handlers::health_check))

        // Video upload (multipart/form-data)
        .route("/video/upload", post(video_handlers::upload_video_handler))

        // Video trim (POST /video/:id/trim, expects { "start": "...", "end": "..." } in JSON body)
        .route("/video/:id/trim", post(video_handlers::trim_video_handler))

        // Add subtitles (POST /video/:id/subtitles, expects { "subtitleText": "...", "start": "...", "end": "..." })
        .route("/video/:id/subtitles", post(video_handlers::add_subtitles_handler))

        // Render video (POST /video/:id/render)
        .route("/video/:id/render", post(video_handlers::render_video_handler))

        // Download rendered video (GET /video/:id/download)
        .route("/video/:id/download", get(video_handlers::download_video_handler))

        // Attach the service as state for all handlers
        .with_state(video_service)
}
