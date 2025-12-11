//! Port (interface) for Flower Repository

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::DomainResult;
use crate::domain::flower::Flower;
use crate::domain::shared::Pagination;

/// Repository trait for Flower entity
#[async_trait]
pub trait FlowerRepository: Send + Sync {
    /// Find a flower by its ID
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Option<Flower>>;

    /// Find all flowers with pagination
    async fn find_all(&self, pagination: &Pagination) -> DomainResult<Vec<Flower>>;

    /// Count total flowers
    async fn count(&self) -> DomainResult<i64>;

    /// Search flowers by name or color
    async fn search(
        &self,
        query: Option<&str>,
        color: Option<&str>,
        pagination: &Pagination,
    ) -> DomainResult<Vec<Flower>>;

    /// Count flowers matching search criteria
    async fn count_search(&self, query: Option<&str>, color: Option<&str>) -> DomainResult<i64>;

    /// Create a new flower
    async fn create(&self, flower: &Flower) -> DomainResult<Flower>;

    /// Update an existing flower
    async fn update(&self, flower: &Flower) -> DomainResult<Flower>;

    /// Delete a flower by ID
    async fn delete(&self, id: Uuid) -> DomainResult<()>;
}
