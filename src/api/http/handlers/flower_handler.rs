//! Flower HTTP Handlers

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;
use validator::Validate;

use crate::api::http::state::AppState;
use crate::application::dtos::{
    ApiResponse, ApiResponseFlower, ApiResponsePaginatedFlower, CreateFlowerRequest, ErrorResponse,
    FlowerResponse, ListFlowersQuery, UpdateFlowerRequest,
};
use crate::domain::errors::{DomainResult, AppError};
use crate::domain::shared::Pagination;

/// Get a flower by ID
#[utoipa::path(
    get,
    path = "/api/flowers/{id}",
    tag = "Flowers",
    params(
        ("id" = Uuid, Path, description = "Flower unique identifier")
    ),
    responses(
        (status = 200, description = "Flower found", body = ApiResponseFlower),
        (status = 404, description = "Flower not found", body = ErrorResponse)
    )
)]
pub async fn get_flower(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> DomainResult<Json<ApiResponse<FlowerResponse>>> {
    let flower = state.flower_usecase.get_flower(id).await?;
    Ok(Json(ApiResponse::success(flower)))
}

/// List all flowers with pagination and optional filters
#[utoipa::path(
    get,
    path = "/api/flowers",
    tag = "Flowers",
    params(ListFlowersQuery),
    responses(
        (status = 200, description = "List of flowers", body = ApiResponsePaginatedFlower)
    )
)]
pub async fn list_flowers(
    State(state): State<AppState>,
    Query(query): Query<ListFlowersQuery>,
) -> DomainResult<Json<ApiResponse<crate::domain::shared::PaginatedResponse<FlowerResponse>>>> {
    let pagination = Pagination {
        page: query.page.unwrap_or(1),
        per_page: query.per_page.unwrap_or(10),
    };

    let result = if query.search.is_some() || query.color.is_some() {
        state
            .flower_usecase
            .search_flowers(query.search, query.color, pagination)
            .await?
    } else {
        state.flower_usecase.list_flowers(pagination).await?
    };

    Ok(Json(ApiResponse::success(result)))
}

/// Create a new flower
#[utoipa::path(
    post,
    path = "/api/flowers",
    tag = "Flowers",
    request_body = CreateFlowerRequest,
    responses(
        (status = 201, description = "Flower created successfully", body = ApiResponseFlower),
        (status = 400, description = "Invalid request data", body = ErrorResponse)
    )
)]
pub async fn create_flower(
    State(state): State<AppState>,
    Json(request): Json<CreateFlowerRequest>,
) -> DomainResult<(StatusCode, Json<ApiResponse<FlowerResponse>>)> {
    // Validate the request first
    request.validate().map_err(|e| AppError::validation(
        e.field_errors()
            .iter()
            .map(|(field, errors)| {
                errors
                    .iter()
                    .map(|error| format!("{}: {}", field, error.message.clone().unwrap_or_else(|| "Invalid input".into())))
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect::<Vec<String>>()
            .join(", ")
    ))?;

    let flower = state.flower_usecase.create_flower(request).await?;
    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::with_message(
            flower,
            "Flower created successfully",
        )),
    ))
}

/// Update an existing flower
#[utoipa::path(
    put,
    path = "/api/flowers/{id}",
    tag = "Flowers",
    params(
        ("id" = Uuid, Path, description = "Flower unique identifier")
    ),
    request_body = UpdateFlowerRequest,
    responses(
        (status = 200, description = "Flower updated successfully", body = ApiResponseFlower),
        (status = 404, description = "Flower not found", body = ErrorResponse),
        (status = 400, description = "Invalid request data", body = ErrorResponse)
    )
)]
pub async fn update_flower(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateFlowerRequest>,
) -> DomainResult<Json<ApiResponse<FlowerResponse>>> {
    // Validate the request first
    request.validate().map_err(|e| AppError::validation(
        e.field_errors()
            .iter()
            .map(|(field, errors)| {
                errors
                    .iter()
                    .map(|error| format!("{}: {}", field, error.message.clone().unwrap_or_else(|| "Invalid input".into())))
                    .collect::<Vec<String>>()
            })
            .flatten()
            .collect::<Vec<String>>()
            .join(", ")
    ))?;

    let flower = state.flower_usecase.update_flower(id, request).await?;
    Ok(Json(ApiResponse::with_message(
        flower,
        "Flower updated successfully",
    )))
}

/// Delete a flower
#[utoipa::path(
    delete,
    path = "/api/flowers/{id}",
    tag = "Flowers",
    params(
        ("id" = Uuid, Path, description = "Flower unique identifier")
    ),
    responses(
        (status = 204, description = "Flower deleted successfully"),
        (status = 404, description = "Flower not found", body = ErrorResponse)
    )
)]
pub async fn delete_flower(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> DomainResult<StatusCode> {
    state.flower_usecase.delete_flower(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
