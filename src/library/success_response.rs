use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// Standard success response structure
#[derive(Serialize)]
pub struct SuccessBody<T: Serialize> {
    pub success: bool,
    pub message: String,
    pub data: Option<T>,
}

pub struct SuccessResponse;

impl SuccessResponse {
    /// Constructs a generic success response
    pub fn create_success<T: Serialize>(
        status_code: StatusCode,
        message: &str,
        data: Option<T>,
    ) -> Response {
        let body = SuccessBody {
            success: true,
            message: message.to_string(),
            data,
        };
        (status_code, Json(body)).into_response()
    }

    /// 200 OK response
    pub fn ok<T: Serialize>(message: Option<&str>, data: Option<T>) -> Response {
        Self::create_success(
            StatusCode::OK,
            message.unwrap_or("The request was successful"),
            data,
        )
    }

    /// 201 Created response
    pub fn created<T: Serialize>(message: Option<&str>, data: Option<T>) -> Response {
        Self::create_success(
            StatusCode::CREATED,
            message.unwrap_or("Resource created successfully"),
            data,
        )
    }

    // 204 No Content response
    // pub fn no_content() -> Response {
    //     StatusCode::NO_CONTENT.into_response()
    // }
}
