use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// Structure for API error response
#[derive(Debug, Serialize)]
pub struct ErrorMessage {
    pub status: u16,
    pub error: String,
}

/// Custom application error implementing IntoResponse for Axum
#[derive(Debug)]
pub struct AppError {
    pub status_code: StatusCode,
    pub message: String,
}

impl AppError {
    pub fn create_error(message: &str, status_code: StatusCode) -> Self {
        Self {
            status_code,
            message: message.to_string(),
        }
    }

    pub fn bad_request(message: Option<&str>) -> Self {
        Self::create_error(
            message.unwrap_or("Bad request, missing required fields"),
            StatusCode::BAD_REQUEST,
        )
    }

    // pub fn unauthorized(message: Option<&str>) -> Self {
    //     Self::create_error(
    //         message.unwrap_or("You are not authorized to access this resource"),
    //         StatusCode::UNAUTHORIZED,
    //     )
    // }

    // pub fn forbidden(message: Option<&str>) -> Self {
    //     Self::create_error(
    //         message.unwrap_or("Unauthorized"),
    //         StatusCode::FORBIDDEN,
    //     )
    // }

    // pub fn conflict(message: Option<&str>) -> Self {
    //     Self::create_error(
    //         message.unwrap_or("The resource already exists and cannot be created again"),
    //         StatusCode::CONFLICT,
    //     )
    // }

    // pub fn too_many_requests(message: Option<&str>) -> Self {
    //     Self::create_error(
    //         message.unwrap_or("Too many requests in a stipulated time frame"),
    //         StatusCode::TOO_MANY_REQUESTS,
    //     )
    // }

    pub fn internal_server(message: Option<&str>) -> Self {
        Self::create_error(
            message.unwrap_or("Internal server error"),
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    }

    // pub fn service_unavailable(message: Option<&str>) -> Self {
    //     Self::create_error(
    //         message.unwrap_or("Service unavailable"),
    //         StatusCode::SERVICE_UNAVAILABLE,
    //     )
    // }

    pub fn not_found(message: Option<&str>) -> Self {
        Self::create_error(
            message.unwrap_or("Not Found"),
            StatusCode::NOT_FOUND,
        )
    }

    // pub fn gone(message: Option<&str>) -> Self {
    //     Self::create_error(
    //         message.unwrap_or("Gone"),
    //         StatusCode::GONE,
    //     )
    // }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = Json(ErrorMessage {
            status: self.status_code.as_u16(),
            error: self.message,
        });
        (self.status_code, body).into_response()
    }
}
