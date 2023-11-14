use axum::{Json, extract::{State, Path}};
use axum::http::StatusCode;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::{app::JsonResult, AppState, AppError};

/// Represents the "category" of a [`Thing`](crate::Thing).
#[derive(Serialize, Deserialize, FromRow, Clone, Eq, PartialEq, Debug)]
pub struct Category {
    pub id: i32,
    pub name: String,
}

/// Request to create a [`Category`].
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct CreateRequest {
    pub name: String,
}

/// Response to creating a [`Category`].
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct CreateResponse {
    pub category: Category
}

/// Creates a category.
pub async fn create(state: State<AppState>, request: Json<CreateRequest>) -> JsonResult<CreateResponse> {

    // Checks for duplicate
    let category_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM category WHERE name=$1 AND deleted IS NULL")
        .bind(&request.name)
        .fetch_one(&state.pool)
        .await?;
    if category_count.0 != 0 {
        return Err(AppError::DuplicateRecord);
    }

    // Inserts new category
    let request_name = request.name.trim().to_lowercase();
    let category: Category = sqlx::query_as("INSERT INTO category (name) VALUES ($1) RETURNING id, name")
        .bind(&request_name)
        .fetch_one(&state.pool)
        .await?;
    let response = CreateResponse { category };
    Ok((StatusCode::CREATED, Json(response)))
}

/// Gets a single category.
pub async fn single(state: State<AppState>, path: Path<i32>) -> JsonResult<Category> {
    let category: Option<Category> = sqlx::query_as("SELECT id, name FROM category WHERE id=$1 AND deleted IS NULL")
        .bind(path.0)
        .fetch_optional(&state.pool)
        .await?;
    let Some(category) = category else {
        return Err(AppError::CategoryNotFound);
    };
    Ok((StatusCode::OK, Json(category)))
}

/// Gets a single category.
pub async fn list(state: State<AppState>) -> JsonResult<Vec<Category>> {
    let categories: Vec<Category> = sqlx::query_as("SELECT id, name FROM category WHERE deleted IS NULL")
        .fetch_all(&state.pool)
        .await?;
    Ok((StatusCode::OK, Json(categories)))
}