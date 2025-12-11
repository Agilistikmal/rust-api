//! Flower Domain Specific Errors

use uuid::Uuid;

use crate::domain::errors::AppError;

/// Flower-specific error constructors
pub struct FlowerError;

impl FlowerError {
    pub fn not_found(id: Uuid) -> AppError {
        AppError::not_found(format!("Flower not found with id: {}", id))
    }

    pub fn invalid_name(reason: impl Into<String>) -> AppError {
        AppError::validation(format!("Invalid flower name: {}", reason.into()))
    }

    pub fn invalid_color(reason: impl Into<String>) -> AppError {
        AppError::validation(format!("Invalid flower color: {}", reason.into()))
    }

    pub fn insufficient_stock() -> AppError {
        AppError::validation("Insufficient stock".to_string())
    }
}
