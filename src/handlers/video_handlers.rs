use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
    body::Body,
    http::{header, StatusCode, HeaderMap, HeaderValue}
};
use axum_extra::extract::Multipart;
use tokio_util::io::ReaderStream;
use std::sync::Arc;
use crate::services::video_service::VideoService;
use crate::library::error_response::AppError;
use crate::library::success_response::SuccessResponse;
use serde_json::json;
// use std::io;

pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok"
    }))
}

pub async fn upload_video_handler(
    State(service): State<Arc<VideoService>>,
    mut multipart: Multipart,
) -> Result<Response, AppError> {
    let Some(field) = multipart.next_field().await.map_err(|e| AppError::bad_request(Some(&e.to_string())))? else {
        return Err(AppError::bad_request(Some("No file uploaded")));
    };
    let file_name = field.file_name().ok_or(AppError::bad_request(Some("No filename provided")))?.to_string();
    let data = field.bytes().await.map_err(|e| AppError::bad_request(Some(&e.to_string())))?;
    let file_path = format!("uploads/{}", file_name);
    let file_size = data.len() as i64;
    tokio::fs::write(&file_path, &data)
        .await
        .map_err(|e| AppError::internal_server(Some(&e.to_string())))?;
    service.upload_video(&file_path, file_name, file_size)
        .await
        .map_err(|e| AppError::internal_server(Some(&e.to_string())))
        .map(|video| SuccessResponse::created(Some("Video uploaded successfully"), Some(video)))
}

pub async fn trim_video_handler(
    State(service): State<Arc<VideoService>>,
    Path(id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Response, AppError> {
    let start = payload["start"].as_str().ok_or(AppError::bad_request(Some("Missing start time")))?;
    let end = payload["end"].as_str().ok_or(AppError::bad_request(Some("Missing end time")))?;
    service.trim_video(id, start, end)
        .await
        .map_err(|e| AppError::internal_server(Some(&e.to_string())))
        .map(|video| SuccessResponse::ok(Some("Video trimmed successfully"), Some(video)))
}

pub async fn add_subtitles_handler(
    State(service): State<Arc<VideoService>>,
    Path(id): Path<i32>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Response, AppError> {
    let subtitle_text = payload["subtitleText"].as_str().ok_or(AppError::bad_request(Some("Missing subtitle text")))?;
    let start = payload["start"].as_str().ok_or(AppError::bad_request(Some("Missing start time")))?;
    let end = payload["end"].as_str().ok_or(AppError::bad_request(Some("Missing end time")))?;
    service.add_subtitles(id, subtitle_text, start, end)
        .await
        .map_err(|e| AppError::internal_server(Some(&e.to_string())))
        .map(|video| SuccessResponse::ok(Some("Subtitles added successfully"), Some(video)))
}

pub async fn render_video_handler(
    State(service): State<Arc<VideoService>>,
    Path(id): Path<i32>,
) -> Result<Response, AppError> {
    service.render_video(id)
        .await
        .map_err(|e| AppError::internal_server(Some(&e.to_string())))
        .map(|video| SuccessResponse::ok(Some("Video rendered successfully"), Some(video)))
}

// pub async fn download_video_handler(
//     State(service): State<Arc<VideoService>>,
//     Path(id): Path<i32>,
// ) -> Result<Response, AppError> {
//     let path = service.get_video_download_path(id)
//         .await
//         .map_err(|e| AppError::internal_server(Some(&e.to_string())))?;
//     let file = NamedFile::open(&path)
//         .await
//         .map_err(|e| AppError::internal_server(Some(&e.to_string())))?;
//     Ok(file.into_response())
// }1
pub async fn download_video_handler(
    State(service): State<Arc<VideoService>>,
    Path(id): Path<i32>,
) -> Result<Response, AppError> {
    let path = service.get_video_download_path(id)
        .await
        .map_err(|e| AppError::internal_server(Some(&e.to_string())))?;

    let file = tokio::fs::File::open(&path)
        .await
        .map_err(|e| AppError::internal_server(Some(&e.to_string())))?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream); // axum::body::Body

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!(
            "attachment; filename=\"{}\"",
            path.file_name().unwrap().to_string_lossy()
        )).unwrap(),
    );

    Ok((StatusCode::OK, headers, body).into_response())
}