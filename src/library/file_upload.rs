// use axum::{
//     response::{IntoResponse, Json},
//     http::StatusCode,
// };

// use axum_extra::extract::Multipart;
// use std::fs;
// use std::path::PathBuf;
// use tokio::{fs::File, io::AsyncWriteExt};
// use uuid::Uuid;
// use serde_json::json;

// // Constants
// const UPLOAD_DIR: &str = "uploads";
// const MAX_FILE_SIZE: usize = 100 * 1024 * 1024; // 100MB
// const ALLOWED_EXTENSIONS: [&str; 4] = ["mp4", "mov", "mkv", "avi"];
// const ALLOWED_MIME_TYPES: [&str; 4] = [
//     "video/mp4",
//     "video/quicktime",
//     "video/x-matroska",
//     "video/x-msvideo"
// ];

// // Handler function
// pub async fn upload_video_handler(mut multipart: Multipart) -> impl IntoResponse {
//     // Ensure upload directory exists
//     if !std::path::Path::new(UPLOAD_DIR).exists() {
//         if let Err(e) = fs::create_dir_all(UPLOAD_DIR) {
//             return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": format!("Failed to create upload dir: {}", e)})));
//         }
//     }

//     // Process the uploaded file
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
