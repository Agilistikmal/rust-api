//! Database Configuration

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::domain::errors::{AppError, DomainResult};

/// Database pool wrapper
#[derive(Clone)]
pub struct DatabasePool {
    pool: PgPool,
}

impl DatabasePool {
    /// Create a new database pool
    pub async fn new(database_url: &str) -> DomainResult<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await
            .map_err(|e| AppError::internal(format!("Failed to connect to database: {}", e)))?;

        Ok(Self { pool })
    }

    /// Get a reference to the pool
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Run migrations
    pub async fn run_migrations(&self) -> DomainResult<()> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await
            .map_err(|e| AppError::internal(format!("Failed to run migrations: {}", e)))?;

        Ok(())
    }
}
