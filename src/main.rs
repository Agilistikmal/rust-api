mod api;
mod application;
mod domain;
mod infrastructure;

use std::sync::Arc;

use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::api::http::{AppState, create_router};
use crate::application::usecases::FlowerUseCase;
use crate::infrastructure::config::AppConfig;
use crate::infrastructure::persistance::{DatabasePool, PostgresFlowerRepository};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = AppConfig::from_env();
    tracing::info!("Starting server on {}", config.server_addr());

    // Initialize database
    tracing::info!("Connecting to database...");
    let db_pool = DatabasePool::new(&config.database_url).await?;

    // Run migrations
    tracing::info!("Running migrations...");
    db_pool.run_migrations().await?;
    tracing::info!("Migrations completed successfully");

    // Setup repositories
    let flower_repository = Arc::new(PostgresFlowerRepository::new(db_pool));

    // Setup use cases
    let flower_usecase = Arc::new(FlowerUseCase::new(flower_repository));

    // Create application state
    let app_state = AppState::new(flower_usecase);

    // Setup CORS
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Create router
    let app = create_router(app_state)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    // Start server
    let listener = tokio::net::TcpListener::bind(&config.server_addr()).await?;
    tracing::info!(
        "🌸 Flower API is running on http://{}",
        config.server_addr()
    );
    tracing::info!(
        "📚 OpenAPI docs available at http://{}/openapi",
        config.server_addr()
    );

    axum::serve(listener, app).await?;

    Ok(())
}
