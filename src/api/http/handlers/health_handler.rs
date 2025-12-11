//! Health Check HTTP Handlers

use axum::Json;

use crate::application::dtos::ApiResponse;

/// Health check endpoint
/// GET /health
pub async fn health_check() -> Json<ApiResponse<&'static str>> {
    Json(ApiResponse::success("OK"))
}
