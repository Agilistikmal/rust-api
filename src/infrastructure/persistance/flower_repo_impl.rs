//! PostgreSQL implementation of FlowerRepository

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::FromRow;
use uuid::Uuid;

use crate::application::ports::FlowerRepository;
use crate::domain::errors::{AppError, DomainResult};
use crate::domain::flower::Flower;
use crate::domain::shared::Pagination;
use crate::infrastructure::persistance::DatabasePool;

/// Database row representation for Flower
#[derive(Debug, FromRow)]
struct FlowerRow {
    id: Uuid,
    name: String,
    color: String,
    description: Option<String>,
    price: f64,
    stock: i32,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl TryFrom<FlowerRow> for Flower {
    type Error = AppError;

    fn try_from(row: FlowerRow) -> Result<Self, Self::Error> {
        Flower::from_persistence(
            row.id,
            row.name,
            row.color,
            row.description,
            row.price,
            row.stock,
            row.created_at,
            row.updated_at,
        )
    }
}

/// PostgreSQL implementation of FlowerRepository
pub struct PostgresFlowerRepository {
    db: DatabasePool,
}

impl PostgresFlowerRepository {
    pub fn new(db: DatabasePool) -> Self {
        Self { db }
    }
}

#[async_trait]
impl FlowerRepository for PostgresFlowerRepository {
    async fn find_by_id(&self, id: Uuid) -> DomainResult<Option<Flower>> {
        let result = sqlx::query_as::<_, FlowerRow>(
            r#"
            SELECT id, name, color, description, price, stock, created_at, updated_at
            FROM flowers
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.db.pool())
        .await?;

        match result {
            Some(row) => Ok(Some(row.try_into()?)),
            None => Ok(None),
        }
    }

    async fn find_all(&self, pagination: &Pagination) -> DomainResult<Vec<Flower>> {
        let rows = sqlx::query_as::<_, FlowerRow>(
            r#"
            SELECT id, name, color, description, price, stock, created_at, updated_at
            FROM flowers
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(pagination.limit())
        .bind(pagination.offset())
        .fetch_all(self.db.pool())
        .await?;

        rows.into_iter().map(|row| row.try_into()).collect()
    }

    async fn count(&self) -> DomainResult<i64> {
        let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM flowers")
            .fetch_one(self.db.pool())
            .await?;

        Ok(result.0)
    }

    async fn search(
        &self,
        query: Option<&str>,
        color: Option<&str>,
        pagination: &Pagination,
    ) -> DomainResult<Vec<Flower>> {
        let search_pattern = query.map(|q| format!("%{}%", q.to_lowercase()));
        let color_pattern = color.map(|c| c.to_lowercase());

        let rows = sqlx::query_as::<_, FlowerRow>(
            r#"
            SELECT id, name, color, description, price, stock, created_at, updated_at
            FROM flowers
            WHERE ($1::text IS NULL OR LOWER(name) LIKE $1)
              AND ($2::text IS NULL OR LOWER(color) = $2)
            ORDER BY created_at DESC
            LIMIT $3 OFFSET $4
            "#,
        )
        .bind(&search_pattern)
        .bind(&color_pattern)
        .bind(pagination.limit())
        .bind(pagination.offset())
        .fetch_all(self.db.pool())
        .await?;

        rows.into_iter().map(|row| row.try_into()).collect()
    }

    async fn count_search(&self, query: Option<&str>, color: Option<&str>) -> DomainResult<i64> {
        let search_pattern = query.map(|q| format!("%{}%", q.to_lowercase()));
        let color_pattern = color.map(|c| c.to_lowercase());

        let result: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*)
            FROM flowers
            WHERE ($1::text IS NULL OR LOWER(name) LIKE $1)
              AND ($2::text IS NULL OR LOWER(color) = $2)
            "#,
        )
        .bind(&search_pattern)
        .bind(&color_pattern)
        .fetch_one(self.db.pool())
        .await?;

        Ok(result.0)
    }

    async fn create(&self, flower: &Flower) -> DomainResult<Flower> {
        use crate::domain::shared::Entity;

        let row = sqlx::query_as::<_, FlowerRow>(
            r#"
            INSERT INTO flowers (id, name, color, description, price, stock, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, name, color, description, price, stock, created_at, updated_at
            "#,
        )
        .bind(flower.id())
        .bind(flower.name())
        .bind(flower.color())
        .bind(flower.description())
        .bind(flower.price())
        .bind(flower.stock())
        .bind(flower.created_at())
        .bind(flower.updated_at())
        .fetch_one(self.db.pool())
        .await?;

        row.try_into()
    }

    async fn update(&self, flower: &Flower) -> DomainResult<Flower> {
        use crate::domain::shared::Entity;

        let row = sqlx::query_as::<_, FlowerRow>(
            r#"
            UPDATE flowers
            SET name = $2, color = $3, description = $4, price = $5, stock = $6, updated_at = $7
            WHERE id = $1
            RETURNING id, name, color, description, price, stock, created_at, updated_at
            "#,
        )
        .bind(flower.id())
        .bind(flower.name())
        .bind(flower.color())
        .bind(flower.description())
        .bind(flower.price())
        .bind(flower.stock())
        .bind(flower.updated_at())
        .fetch_one(self.db.pool())
        .await?;

        row.try_into()
    }

    async fn delete(&self, id: Uuid) -> DomainResult<()> {
        sqlx::query("DELETE FROM flowers WHERE id = $1")
            .bind(id)
            .execute(self.db.pool())
            .await?;

        Ok(())
    }
}
