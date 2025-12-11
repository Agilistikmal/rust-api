//! HTTP API Application State

use std::sync::Arc;

use crate::application::usecases::FlowerUseCase;
use crate::infrastructure::persistance::PostgresFlowerRepository;

/// Shared application state for HTTP handlers
#[derive(Clone)]
pub struct AppState {
    pub flower_usecase: Arc<FlowerUseCase<PostgresFlowerRepository>>,
    // Future: pub other_usecase: Arc<OtherUseCase<...>>,
}

impl AppState {
    pub fn new(flower_usecase: Arc<FlowerUseCase<PostgresFlowerRepository>>) -> Self {
        Self { flower_usecase }
    }
}
