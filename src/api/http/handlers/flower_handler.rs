//! Flower HTTP Handlers

use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use uuid::Uuid;

use crate::api::http::state::AppState;
use crate::application::dtos::{
    ApiResponse, CreateFlowerRequest, FlowerResponse, ListFlowersQuery, UpdateFlowerRequest,
};
use crate::domain::errors::DomainResult;
use crate::domain::shared::{PaginatedResponse, Pagination};

/// Get a flower by ID
/// GET /api/flowers/:id
pub async fn get_flower(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> DomainResult<Json<ApiResponse<FlowerResponse>>> {
    let flower = state.flower_usecase.get_flower(id).await?;
    Ok(Json(ApiResponse::success(flower)))
}

/// List all flowers with pagination
/// GET /api/flowers
pub async fn list_flowers(
    State(state): State<AppState>,
    Query(query): Query<ListFlowersQuery>,
) -> DomainResult<Json<ApiResponse<PaginatedResponse<FlowerResponse>>>> {
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
/// POST /api/flowers
pub async fn create_flower(
    State(state): State<AppState>,
    Json(request): Json<CreateFlowerRequest>,
) -> DomainResult<(StatusCode, Json<ApiResponse<FlowerResponse>>)> {
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
/// PUT /api/flowers/:id
pub async fn update_flower(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(request): Json<UpdateFlowerRequest>,
) -> DomainResult<Json<ApiResponse<FlowerResponse>>> {
    let flower = state.flower_usecase.update_flower(id, request).await?;
    Ok(Json(ApiResponse::with_message(
        flower,
        "Flower updated successfully",
    )))
}

/// Delete a flower
/// DELETE /api/flowers/:id
pub async fn delete_flower(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> DomainResult<StatusCode> {
    state.flower_usecase.delete_flower(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
