//! Data Transfer Objects for API layer

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::flower::Flower;
use crate::domain::shared::Entity;

/// Response DTO for Flower
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowerResponse {
    pub id: Uuid,
    pub name: String,
    pub color: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub created_at: DateTime<Utc>,
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFlowerRequest {
    pub name: String,
    pub color: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
}

/// Request DTO for updating an existing Flower
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFlowerRequest {
    pub name: Option<String>,
    pub color: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub stock: Option<i32>,
}

/// Query parameters for listing flowers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFlowersQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub search: Option<String>,
    pub color: Option<String>,
}

/// Generic API response wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
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
