use axum::{Json, extract::{State, Path}};
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
pub struct CreateCategoryRequest {
    pub name: String,
}

/// Response to creating a [`Category`].
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct CreateCategoryResponse {
    pub category: Category
}

/// Creates a category.
pub async fn create(state: State<AppState>, request: Json<CreateCategoryRequest>) -> JsonResult<CreateCategoryResponse> {

    // Checks for duplicate
    let category_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM category WHERE name=$1")
        .bind(&request.name)
        .fetch_one(&state.pool)
        .await?;
    if category_count.0 != 0 {
        return Err(AppError::DuplicateCategory);
    }

    // Inserts new category
    let request_name = request.name.trim().to_lowercase();
    let category: Category = sqlx::query_as("INSERT INTO category (name) VALUES ($1) RETURNING id, name")
        .bind(&request_name)
        .fetch_one(&state.pool)
        .await?;
    let response = CreateCategoryResponse { category };
    Ok(Json(response))
}

/// Gets a single category.
pub async fn single(state: State<AppState>, path: Path<i32>) -> JsonResult<Category> {
    let category: Option<Category> = sqlx::query_as("SELECT id, name FROM category WHERE id=$1")
        .bind(path.0)
        .fetch_optional(&state.pool)
        .await?;
    let Some(category) = category else {
        return Err(AppError::RecordNotFound);
    };
    Ok(Json(category))
}