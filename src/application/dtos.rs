//! Data Transfer Objects for API layer

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::domain::flower::Flower;
use crate::domain::shared::Entity;

/// Response DTO for Flower
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "id": "550e8400-e29b-41d4-a716-446655440001",
    "name": "Rose",
    "color": "red",
    "description": "A beautiful red rose",
    "price": 25000.0,
    "stock": 100,
    "created_at": "2024-12-11T00:00:00Z",
    "updated_at": "2024-12-11T00:00:00Z"
}))]
pub struct FlowerResponse {
    /// Unique identifier
    pub id: Uuid,
    /// Flower name
    pub name: String,
    /// Flower color
    pub color: String,
    /// Optional description
    pub description: Option<String>,
    /// Price in IDR
    pub price: f64,
    /// Available stock
    pub stock: i32,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl From<Flower> for FlowerResponse {
    fn from(flower: Flower) -> Self {
        Self {
            id: flower.id(),
            name: flower.name().to_string(),
            color: flower.color().to_string(),
            description: flower.description().map(String::from),
            price: flower.price(),
            stock: flower.stock(),
            created_at: flower.created_at(),
            updated_at: flower.updated_at(),
        }
    }
}

/// Request DTO for creating a new Flower
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "name": "Rose",
    "color": "red",
    "description": "A beautiful red rose",
    "price": 25000.0,
    "stock": 100
}))]
pub struct CreateFlowerRequest {
    /// Flower name (max 100 characters)
    pub name: String,
    /// Flower color (max 50 characters)
    pub color: String,
    /// Optional description
    pub description: Option<String>,
    /// Price in IDR
    pub price: f64,
    /// Initial stock quantity
    pub stock: i32,
}

/// Request DTO for updating an existing Flower
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "name": "Red Rose",
    "price": 30000.0,
    "stock": 150
}))]
pub struct UpdateFlowerRequest {
    /// New flower name
    pub name: Option<String>,
    /// New flower color
    pub color: Option<String>,
    /// New description
    pub description: Option<String>,
    /// New price
    pub price: Option<f64>,
    /// New stock quantity
    pub stock: Option<i32>,
}

/// Query parameters for listing flowers
#[derive(Debug, Clone, Serialize, Deserialize, IntoParams)]
pub struct ListFlowersQuery {
    /// Page number (default: 1)
    #[param(minimum = 1, default = 1)]
    pub page: Option<i64>,
    /// Items per page (default: 10)
    #[param(minimum = 1, maximum = 100, default = 10)]
    pub per_page: Option<i64>,
    /// Search by flower name
    pub search: Option<String>,
    /// Filter by color
    pub color: Option<String>,
}

/// Generic API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    /// Indicates if the request was successful
    pub success: bool,
    /// Response data
    pub data: T,
    /// Optional message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data,
            message: None,
        }
    }

    pub fn with_message(data: T, message: impl Into<String>) -> Self {
        Self {
            success: true,
            data,
            message: Some(message.into()),
        }
    }
}

/// API Response for single flower
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiResponseFlower {
    pub success: bool,
    pub data: FlowerResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Paginated flower response for OpenAPI schema
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct PaginatedFlowerResponse {
    pub data: Vec<FlowerResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

/// API Response for paginated flowers
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiResponsePaginatedFlower {
    pub success: bool,
    pub data: PaginatedFlowerResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Error response
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[schema(example = json!({
    "success": false,
    "error": "Flower not found with id: 550e8400-e29b-41d4-a716-446655440001"
}))]
pub struct ErrorResponse {
    /// Always false for errors
    pub success: bool,
    /// Error message
    pub error: String,
}
