use axum::{
    extract::{Path, State},
    response::{IntoResponse, Response},
    Json,
    body::Body,
    http::{header, StatusCode, HeaderMap, HeaderValue}
};
use axum_extra::extract::Multipart;
use tokio_util::io::ReaderStream;
use tokio::{fs, io::AsyncWriteExt};
use std::sync::Arc;
use crate::services::video_service::VideoService;
use crate::library::error_response::AppError;
use crate::library::success_response::SuccessResponse;
use serde_json::json;
use std::path::{Path as StdPath};
use uuid::Uuid;

const UPLOAD_DIR: &str = "uploads";
const MAX_FILE_SIZE: usize = 100 * 1024 * 1024; // 100MB
const ALLOWED_EXTENSIONS: [&str; 4] = ["mp4", "mov", "mkv", "avi"];
const ALLOWED_MIME_TYPES: [&str; 4] = [
    "video/mp4",
    "video/quicktime",
    "video/x-matroska",
    "video/x-msvideo"
];


pub async fn health_check() -> impl IntoResponse {
    Json(json!({
        "status": "ok"
    }))
}

pub async fn upload_video_handler(
    State(service): State<Arc<VideoService>>,
    mut multipart: Multipart,
) -> Result<Response, AppError> {
    // Ensure upload directory exists (async)
    if fs::metadata(UPLOAD_DIR).await.is_err() {
        fs::create_dir_all(UPLOAD_DIR)
            .await
            .map_err(|e| AppError::internal_server(Some(&format!("Failed to create upload dir: {}", e))))?;
    }

    // Process the uploaded file (first field only)
    let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::bad_request(Some(&e.to_string())))?
    else {
        return Err(AppError::bad_request(Some("No file uploaded")));
    };

    let file_name = field
        .file_name()
        .ok_or(AppError::bad_request(Some("No filename provided")))?
        .to_string();

    let content_type = field
        .content_type()
        .map(|ct| ct.to_string())
        .unwrap_or_default();

    // Validate extension and MIME type
    let ext = StdPath::new(&file_name)
        .extension()
        .and_then(|ext| ext.to_str())
        .unwrap_or("")
        .to_lowercase();

    if !ALLOWED_EXTENSIONS.contains(&ext.as_str()) || !ALLOWED_MIME_TYPES.contains(&content_type.as_str()) {
        return Err(AppError::bad_request(Some("Only video files (mp4, mov, mkv, avi) are allowed")));
    }

    // Read the file data
    let data = field
        .bytes()
        .await
        .map_err(|e| AppError::bad_request(Some(&format!("Failed to read file data: {}", e))))?;

    if data.len() > MAX_FILE_SIZE {
        return Err(AppError::bad_request(Some("File exceeds 100MB limit")));
    }

    // Generate a unique filename and save the file
    let new_filename = format!("{}-{}", Uuid::new_v4(), file_name);
    let file_path = format!("{}/{}", UPLOAD_DIR, new_filename);

    let mut file = fs::File::create(&file_path)
        .await
        .map_err(|e| AppError::internal_server(Some(&format!("Failed to create file: {}", e))))?;

    file.write_all(&data)
        .await
        .map_err(|e| AppError::internal_server(Some(&format!("Failed to write file: {}", e))))?;

    // Call the service to record the video in the database
    let video = service
        .upload_video(&file_path, &file_name, data.len() as i64)
        .await
        .map_err(|e| AppError::internal_server(Some(&e.to_string())))?;

    Ok(SuccessResponse::created(Some("Video uploaded successfully"), Some(video)).into_response())
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


pub async fn download_video_handler(
    State(service): State<Arc<VideoService>>,
    Path(id): Path<i32>,
) -> Result<Response, AppError> {
    let path_str = service.get_video_download_path(id)
        .await
        .map_err(|e| AppError::not_found(Some(&e.to_string())))?;

    let file = tokio::fs::File::open(&path_str)
        .await
        .map_err(|e| AppError::not_found(Some(&e.to_string())))?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    let file_name = StdPath::new(&path_str)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("video.mp4");

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("application/octet-stream"),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!("attachment; filename=\"{}\"", file_name)).unwrap(),
    );

    Ok((StatusCode::OK, headers, body).into_response())
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

// pub async fn upload_video_handler(
//     State(service): State<Arc<VideoService>>,
//     mut multipart: Multipart,
// ) -> Result<Response, AppError> {
//     let Some(field) = multipart.next_field().await.map_err(|e| AppError::bad_request(Some(&e.to_string())))? else {
//         return Err(AppError::bad_request(Some("No file uploaded")));
//     };
//     let file_name = field.file_name().ok_or(AppError::bad_request(Some("No filename provided")))?.to_string();
//     let data = field.bytes().await.map_err(|e| AppError::bad_request(Some(&e.to_string())))?;
//     let file_path = format!("uploads/{}", file_name);
//     let file_size = data.len() as i64;
//     tokio::fs::write(&file_path, &data)
//         .await
//         .map_err(|e| AppError::internal_server(Some(&e.to_string())))?;
//     service.upload_video(&file_path, &file_name, file_size)
//         .await
//         .map_err(|e| AppError::internal_server(Some(&e.to_string())))
//         .map(|video| SuccessResponse::created(Some("Video uploaded successfully"), Some(video)))
// }