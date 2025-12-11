//! HTTP Routes configuration

use axum::{
    Router,
    routing::{delete, get, post, put},
};
use utoipa::OpenApi;
use utoipa_scalar::{Scalar, Servable};

use super::handlers::{
    create_flower, delete_flower, get_flower, health_check, list_flowers, update_flower,
};
use super::openapi::ApiDoc;
use super::state::AppState;

/// Create the main HTTP router
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // OpenAPI Scalar UI
        .merge(Scalar::with_url("/openapi", ApiDoc::openapi()))
        // Health check
        .route("/health", get(health_check))
        // API routes
        .nest("/api", api_routes())
        .with_state(state)
}

/// API routes under /api prefix
fn api_routes() -> Router<AppState> {
    Router::new().nest("/flowers", flower_routes())
    // Future: .nest("/other", other_routes())
}

/// Flower routes: /api/flowers
fn flower_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_flowers))
        .route("/", post(create_flower))
        .route("/{id}", get(get_flower))
        .route("/{id}", put(update_flower))
        .route("/{id}", delete(delete_flower))
}
