//! Flower Use Cases

use std::sync::Arc;
use uuid::Uuid;

use crate::application::dtos::{CreateFlowerRequest, FlowerResponse, UpdateFlowerRequest};
use crate::application::ports::FlowerRepository;
use crate::domain::errors::DomainResult;
use crate::domain::flower::{Flower, FlowerError};
use crate::domain::shared::{PaginatedResponse, Pagination};

/// Use case for flower operations
pub struct FlowerUseCase<R: FlowerRepository> {
    repository: Arc<R>,
}

impl<R: FlowerRepository> FlowerUseCase<R> {
    pub fn new(repository: Arc<R>) -> Self {
        Self { repository }
    }

    /// Get a flower by ID
    pub async fn get_flower(&self, id: Uuid) -> DomainResult<FlowerResponse> {
        let flower = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| FlowerError::not_found(id))?;

        Ok(FlowerResponse::from(flower))
    }

    /// List all flowers with pagination
    pub async fn list_flowers(
        &self,
        pagination: Pagination,
    ) -> DomainResult<PaginatedResponse<FlowerResponse>> {
        let flowers = self.repository.find_all(&pagination).await?;
        let total = self.repository.count().await?;

        let flower_responses: Vec<FlowerResponse> =
            flowers.into_iter().map(FlowerResponse::from).collect();

        Ok(PaginatedResponse::new(flower_responses, total, &pagination))
    }

    /// Search flowers
    pub async fn search_flowers(
        &self,
        query: Option<String>,
        color: Option<String>,
        pagination: Pagination,
    ) -> DomainResult<PaginatedResponse<FlowerResponse>> {
        let flowers = self
            .repository
            .search(query.as_deref(), color.as_deref(), &pagination)
            .await?;
        let total = self
            .repository
            .count_search(query.as_deref(), color.as_deref())
            .await?;

        let flower_responses: Vec<FlowerResponse> =
            flowers.into_iter().map(FlowerResponse::from).collect();

        Ok(PaginatedResponse::new(flower_responses, total, &pagination))
    }

    /// Create a new flower
    pub async fn create_flower(
        &self,
        request: CreateFlowerRequest,
    ) -> DomainResult<FlowerResponse> {
        let flower = Flower::new(
            request.name,
            request.color,
            request.description,
            request.price,
            request.stock,
        )?;

        let created_flower = self.repository.create(&flower).await?;
        Ok(FlowerResponse::from(created_flower))
    }

    /// Update an existing flower
    pub async fn update_flower(
        &self,
        id: Uuid,
        request: UpdateFlowerRequest,
    ) -> DomainResult<FlowerResponse> {
        let mut flower = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| FlowerError::not_found(id))?;

        // Apply updates if provided
        if let Some(name) = request.name {
            flower.update_name(name)?;
        }
        if let Some(color) = request.color {
            flower.update_color(color)?;
        }
        if let Some(description) = request.description {
            flower.update_description(Some(description));
        }
        if let Some(price) = request.price {
            flower.update_price(price);
        }
        if let Some(stock) = request.stock {
            flower.update_stock(stock);
        }

        let updated_flower = self.repository.update(&flower).await?;
        Ok(FlowerResponse::from(updated_flower))
    }

    /// Delete a flower
    pub async fn delete_flower(&self, id: Uuid) -> DomainResult<()> {
        // Check if flower exists
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| FlowerError::not_found(id))?;

        self.repository.delete(id).await
    }
}
