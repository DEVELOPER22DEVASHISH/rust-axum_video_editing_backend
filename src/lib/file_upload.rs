// use axum_extra::extract::Multipart;
// use axum::{http::StatusCode, response::IntoResponse, Json};
// use std::path::{Path, PathBuf};
// use tokio::fs;
// use serde::Serialize;

// const ALLOWED_EXTENSIONS: [&str; 4] = ["mp4", "mov", "mkv", "avi"];
// const ALLOWED_MIME_TYPES: [&str; 4] = [
//     "video/mp4",
//     "video/quicktime",
//     "video/x-matroska",
//     "video/x-msvideo",
// ];
// pub const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024; // 100 MB

// #[derive(Serialize)]
// pub struct UploadResponse {
//     pub success: bool,
//     pub message: String,
//     pub file_path: Option<String>,
// }

// /// Ensures the upload directory exists.
// pub async fn ensure_upload_dir(upload_dir: &str) -> std::io::Result<()> {
//     if !Path::new(upload_dir).exists() {
//         fs::create_dir_all(upload_dir).await?;
//     }
//     Ok(())
// }

// /// Handles a multipart file upload.
// /// Returns a JSON response with the file path or an error.
// pub async fn handle_file_upload(
//     mut multipart: Multipart,
//     upload_dir: &str,
// ) -> impl IntoResponse {
//     if let Err(e) = ensure_upload_dir(upload_dir).await {
//         return (
//             StatusCode::INTERNAL_SERVER_ERROR,
//             Json(UploadResponse {
//                 success: false,
//                 message: format!("Failed to create upload directory: {}", e),
//                 file_path: None,
//             }),
//         );
//     }

//     while let Some(field) = multipart.next_field().await.unwrap_or(None) {
//         let file_name = field.file_name().map(|n| n.to_string()).unwrap_or_else(|| "file".to_string());
//         let content_type = field.content_type().map(|ct| ct.to_string()).unwrap_or_default();

//         // Validate extension
//         let ext = Path::new(&file_name)
//             .extension()
//             .and_then(|e| e.to_str())
//             .map(|e| e.to_lowercase())
//             .unwrap_or_else(|| "".to_string());
//         if !ALLOWED_EXTENSIONS.contains(&ext.as_str()) {
//             return (
//                 StatusCode::BAD_REQUEST,
//                 Json(UploadResponse {
//                     success: false,
//                     message: format!("Invalid file extension: {}", ext),
//                     file_path: None,
//                 }),
//             );
//         }

//         // Validate MIME type
//         if !ALLOWED_MIME_TYPES.contains(&content_type.as_str()) {
//             return (
//                 StatusCode::BAD_REQUEST,
//                 Json(UploadResponse {
//                     success: false,
//                     message: format!("Invalid MIME type: {}", content_type),
//                     file_path: None,
//                 }),
//             );
//         }

//         // Validate file size
//         let data = field.bytes().await.unwrap_or_default();
//         if data.len() as u64 > MAX_FILE_SIZE {
//             return (
//                 StatusCode::BAD_REQUEST,
//                 Json(UploadResponse {
//                     success: false,
//                     message: "File too large".to_string(),
//                     file_path: None,
//                 }),
//             );
//         }

//         // Save file
//         let save_name = format!("{}-{}", chrono::Utc::now().timestamp_millis(), file_name);
//         let save_path = PathBuf::from(upload_dir).join(&save_name);
//         if let Err(e) = fs::write(&save_path, &data).await {
//             return (
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(UploadResponse {
//                     success: false,
//                     message: format!("Failed to save file: {}", e),
//                     file_path: None,
//                 }),
//             );
//         }

//         return (
//             StatusCode::OK,
//             Json(UploadResponse {
//                 success: true,
//                 message: "File uploaded successfully".to_string(),
//                 file_path: Some(save_path.to_string_lossy().to_string()),
//             }),
//         );
//     }

//     (
//         StatusCode::BAD_REQUEST,
//         Json(UploadResponse {
//             success: false,
//             message: "No file found in request".to_string(),
//             file_path: None,
//         }),
//     )
// }


// use axum::{
//     extract::Multipart,
//     response::{IntoResponse, Json},
// };
// use std::{fs::create_dir_all, path::PathBuf};
// use tokio::{fs::File, io::AsyncWriteExt};
// use uuid::Uuid;
// use serde_json::json;

// const UPLOAD_DIR: &str = "uploads";
// const MAX_FILE_SIZE: usize = 100 * 1024 * 1024; // 100MB
// const ALLOWED_EXTENSIONS: [&str; 4] = ["mp4", "mov", "mkv", "avi"];
// const ALLOWED_MIME_TYPES: [&str; 4] = [
//     "video/mp4",
//     "video/quicktime",
//     "video/x-matroska",
//     "video/x-msvideo"
// ];

// pub async fn upload_video_handler(mut multipart: Multipart) -> impl IntoResponse {
//     create_dir_all(UPLOAD_DIR).unwrap();

//     while let Some(field) = multipart.next_field().await.unwrap() {
//         let file_name = field.file_name().map(|name| name.to_string()).unwrap_or("video".into());
//         let content_type = field.content_type().map(|ct| ct.to_string()).unwrap_or_default();

//         let ext = PathBuf::from(&file_name)
//             .extension()
//             .and_then(|ext| ext.to_str())
//             .unwrap_or("")
//             .to_lowercase();

//         if !ALLOWED_EXTENSIONS.contains(&ext.as_str()) || !ALLOWED_MIME_TYPES.contains(&content_type.as_str()) {
//             return Json(json!({ "error": "Only video files (mp4, mov, mkv, avi) are allowed" }));
//         }

//         let data = field.bytes().await.unwrap();
//         if data.len() > MAX_FILE_SIZE {
//             return Json(json!({ "error": "File exceeds 100MB limit" }));
//         }

//         let new_filename = format!("{}-{}", Uuid::new_v4(), file_name);
//         let filepath = format!("{}/{}", UPLOAD_DIR, new_filename);
//         let mut file = File::create(&filepath).await.unwrap();
//         file.write_all(&data).await.unwrap();

//         return Json(json!({
//             "success": true,
//             "filename": new_filename,
//             "path": filepath
//         }));
//     }

//     Json(json!({ "error": "No file uploaded" }))
// }


// use axum::{
//     extract::Multipart,
//     response::{IntoResponse, Json},
//     http::StatusCode,
// };
// use std::{fs::create_dir_all, path::PathBuf};
// use tokio::{fs::File, io::AsyncWriteExt};
// use uuid::Uuid;
// use serde_json::json;

// const UPLOAD_DIR: &str = "uploads";
// const MAX_FILE_SIZE: usize = 100 * 1024 * 1024; // 100MB
// const ALLOWED_EXTENSIONS: [&str; 4] = ["mp4", "mov", "mkv", "avi"];
// const ALLOWED_MIME_TYPES: [&str; 4] = [
//     "video/mp4",
//     "video/quicktime",
//     "video/x-matroska",
//     "video/x-msvideo"
// ];

// pub async fn upload_video_handler(mut multipart: Multipart) -> impl IntoResponse {
//     if let Err(e) = create_dir_all(UPLOAD_DIR) {
//         return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": format!("Failed to create upload dir: {}", e) })));
//     }

//     while let Some(field) = match multipart.next_field().await {
//         Ok(f) => f,
//         Err(_) => return (StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid multipart data" }))),
//     } {
//         let file_name = field.file_name().map(|name| name.to_string()).unwrap_or("video".into());
//         let content_type = field.content_type().map(|ct| ct.to_string()).unwrap_or_default();

//         let ext = PathBuf::from(&file_name)
//             .extension()
//             .and_then(|ext| ext.to_str())
//             .unwrap_or("")
//             .to_lowercase();

//         if !ALLOWED_EXTENSIONS.contains(&ext.as_str()) || !ALLOWED_MIME_TYPES.contains(&content_type.as_str()) {
//             return (StatusCode::BAD_REQUEST, Json(json!({ "error": "Only video files (mp4, mov, mkv, avi) are allowed" })));
//         }

//         let data = match field.bytes().await {
//             Ok(d) => d,
//             Err(_) => return (StatusCode::BAD_REQUEST, Json(json!({ "error": "Failed to read file data" }))),
//         };
//         if data.len() > MAX_FILE_SIZE {
//             return (StatusCode::BAD_REQUEST, Json(json!({ "error": "File exceeds 100MB limit" })));
//         }

//         let new_filename = format!("{}-{}", Uuid::new_v4(), file_name);
//         let filepath = format!("{}/{}", UPLOAD_DIR, new_filename);
//         match File::create(&filepath).await {
//             Ok(mut file) => {
//                 if let Err(_) = file.write_all(&data).await {
//                     return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to write file" })));
//                 }
//             }
//             Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to create file" }))),
//         }

//         return (StatusCode::OK, Json(json!({
//             "success": true,
//             "filename": new_filename,
//             "path": filepath
//         })));
//     }

//     (StatusCode::BAD_REQUEST, Json(json!({ "error": "No file uploaded" })))
// }


use axum::{
    extract::Multipart,
    response::{IntoResponse, Json},
    http::StatusCode,
};
use std::fs;
use std::path::PathBuf;
use tokio::{fs::File, io::AsyncWriteExt};
use uuid::Uuid;
use serde_json::json;

// Constants
const UPLOAD_DIR: &str = "uploads";
const MAX_FILE_SIZE: usize = 100 * 1024 * 1024; // 100MB
const ALLOWED_EXTENSIONS: [&str; 4] = ["mp4", "mov", "mkv", "avi"];
const ALLOWED_MIME_TYPES: [&str; 4] = [
    "video/mp4",
    "video/quicktime",
    "video/x-matroska",
    "video/x-msvideo"
];

// Handler function
pub async fn upload_video_handler(mut multipart: Multipart) -> impl IntoResponse {
    // Ensure upload directory exists
    if !std::path::Path::new(UPLOAD_DIR).exists() {
        if let Err(e) = fs::create_dir_all(UPLOAD_DIR) {
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Failed to create upload dir: {}", e)})));
        }
    }

    // Process the uploaded file
    while let Some(field) = match multipart.next_field().await {
        Ok(f) => f,
        Err(_) => return (StatusCode::BAD_REQUEST, Json(json!({ "error": "Invalid multipart data" }))),
    } {
        let file_name = field.file_name().map(|name| name.to_string()).unwrap_or("video".into());
        let content_type = field.content_type().map(|ct| ct.to_string()).unwrap_or_default();

        let ext = PathBuf::from(&file_name)
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        if !ALLOWED_EXTENSIONS.contains(&ext.as_str()) || !ALLOWED_MIME_TYPES.contains(&content_type.as_str()) {
            return (StatusCode::BAD_REQUEST, Json(json!({ "error": "Only video files (mp4, mov, mkv, avi) are allowed" })));
        }

        let data = match field.bytes().await {
            Ok(d) => d,
            Err(_) => return (StatusCode::BAD_REQUEST, Json(json!({ "error": "Failed to read file data" }))),
        };
        if data.len() > MAX_FILE_SIZE {
            return (StatusCode::BAD_REQUEST, Json(json!({ "error": "File exceeds 100MB limit" })));
        }

        let new_filename = format!("{}-{}", Uuid::new_v4(), file_name);
        let filepath = format!("{}/{}", UPLOAD_DIR, new_filename);
        match File::create(&filepath).await {
            Ok(mut file) => {
                if let Err(_) = file.write_all(&data).await {
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to write file" })));
                }
            }
            Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": "Failed to create file" }))),
        }

        return (StatusCode::OK, Json(json!({
            "success": true,
            "filename": new_filename,
            "path": filepath
        })));
    }

    (StatusCode::BAD_REQUEST, Json(json!({ "error": "No file uploaded" })))
}
