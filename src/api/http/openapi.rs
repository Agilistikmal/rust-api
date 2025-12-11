//! OpenAPI Documentation Configuration

use utoipa::OpenApi;

use crate::api::http::handlers::{flower_handler, health_handler};
use crate::application::dtos::{
    ApiResponseFlower, ApiResponsePaginatedFlower, CreateFlowerRequest, ErrorResponse,
    FlowerResponse, PaginatedFlowerResponse, UpdateFlowerRequest,
};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Flower API",
        version = "1.0.0",
        description = "RESTful API for managing flower data",
        contact(
            name = "API Support",
            email = "support@example.com"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    servers(
        (url = "http://localhost:3000", description = "Local development server")
    ),
    tags(
        (name = "Health", description = "Health check endpoints"),
        (name = "Flowers", description = "Flower management endpoints")
    ),
    paths(
        health_handler::health_check,
        flower_handler::get_flower,
        flower_handler::list_flowers,
        flower_handler::create_flower,
        flower_handler::update_flower,
        flower_handler::delete_flower,
    ),
    components(
        schemas(
            health_handler::HealthResponse,
            FlowerResponse,
            CreateFlowerRequest,
            UpdateFlowerRequest,
            ErrorResponse,
            ApiResponseFlower,
            ApiResponsePaginatedFlower,
            PaginatedFlowerResponse,
        )
    )
)]
pub struct ApiDoc;
