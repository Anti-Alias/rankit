use axum::{Extension, Json, extract::{State, Path}};
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use crate::{account::Claims, app::{JsonResult, AppState, AppError}};

/// Represents the "category" of a [`Thing`](crate::Thing).
#[derive(Serialize, Deserialize, FromRow, Clone, Eq, PartialEq, Debug)]
pub struct Category {
    pub id: i32,
    pub account_id: i32,
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
pub async fn create(state: State<AppState>, claims: Extension<Claims>, request: Json<CreateCategoryRequest>) -> JsonResult<CreateCategoryResponse> {

    // Checks for duplicate
    let category_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM category WHERE account_id=$1 AND name=$2")
        .bind(claims.id)
        .bind(&request.name)
        .fetch_one(&state.pool)
        .await?;
    if category_count.0 != 0 {
        return Err(AppError::DuplicateCategory);
    }

    // Inserts new category
    let request_name = request.name.trim().to_lowercase();
    let category: Category = sqlx::query_as("INSERT INTO category (account_id, name) VALUES ($1, $2) RETURNING id, account_id, name")
        .bind(claims.id)
        .bind(&request_name)
        .fetch_one(&state.pool)
        .await?;
    let response = CreateCategoryResponse { category };
    Ok(Json(response))
}

/// Gets a single category.
pub async fn single(state: State<AppState>, path: Path<i32>) -> JsonResult<Category> {
    let category: Option<Category> = sqlx::query_as("SELECT id, account_id, name FROM category WHERE id=$1")
        .bind(path.0)
        .fetch_optional(&state.pool)
        .await?;
    let Some(category) = category else {
        return Err(AppError::RecordNotFound);
    };
    Ok(Json(category))
}

/// List all categories under this account.
pub async fn list(state: State<AppState>, claims: Extension<Claims>) -> JsonResult<Vec<Category>> {
    let categories: Vec<Category> = sqlx::query_as("SELECT id, account_id, name FROM category WHERE account_id=$1")
        .bind(claims.id)
        .fetch_all(&state.pool)
        .await?;
    Ok(Json(categories))
}