use axum::{Json, extract::{State, Path}};
use axum::http::StatusCode;
use serde::{Serialize, Deserialize};
use sqlx::prelude::*;
use crate::{app::JsonResult, AppState, AppError, rank::RankedThing};

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

/// Statistics of a [`Category`].
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Statistics {
    pub category: Category,
    pub things: Vec<RankedThing>
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


/// Deletes a category.
pub async fn delete(state: State<AppState>, path: Path<i32>) -> Result<StatusCode, AppError> {
    let category_id = path.0;

    log::trace!("Checking that category {category_id} exists");
    let category_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM category WHERE id=$1 AND deleted IS NULL")
        .bind(category_id)
        .fetch_one(&state.pool)
        .await?;
    if category_count.0 == 0 {
        return Err(AppError::CategoryNotFound);
    }

    let mut transaction = state.pool.begin().await?;
    let conn = transaction.acquire().await?;

    log::trace!("Deleting category {category_id}");
    sqlx::query("UPDATE category SET deleted=NOW() WHERE id=$1 AND deleted IS NULL")
        .bind(&category_id)
        .execute(&mut *conn)
        .await
        .map_err(|_| AppError::CategoryNotFound)?;

    log::trace!("Deleting ranks associated with category {category_id}");
    sqlx::query("UPDATE rank SET deleted=NOW() WHERE category_id=$1 AND deleted IS NULL")
        .bind(category_id)
        .execute(conn)
        .await?;

    transaction.commit().await?;
    Ok(StatusCode::NO_CONTENT)
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

const LIST_RANKED_THINGS_FOR_CATEGORY: &str = "\
    SELECT \
        r.id, \
        r.thing_id, \
        r.category_id, \
        r.score, \
        r.run, \
        t.id, \
        t.name, \
        t.file \
    FROM \
        rank r \
        JOIN thing t ON r.thing_id = t.id \
        JOIN category c ON r.category_id = c.id \
    WHERE \
        r.category_id = $1 \
        AND r.deleted IS NULL \
    ORDER BY \
        r.score DESC\
";

/// Gets statistics of a [`Category`].
pub async fn statistics(state: State<AppState>, path: Path<i32>) -> JsonResult<Statistics> {
    let category_id = path.0;

    log::trace!("Fetching category {category_id}");
    let category: Option<Category> = sqlx::query_as("SELECT id, name FROM category WHERE id=$1 AND deleted IS NULL")
        .bind(category_id)
        .fetch_optional(&state.pool)
        .await?;
    let Some(category) = category else {
        return Err(AppError::CategoryNotFound);
    };

    log::trace!("Fetching statistics for category {category_id}");
    let thing_rows: Vec<(i32, i32, i32, f64, i32, i32, String, String)> = sqlx::query_as(LIST_RANKED_THINGS_FOR_CATEGORY)
        .bind(category_id)
        .fetch_all(&state.pool)
        .await?;
    let things: Vec<RankedThing> = thing_rows.into_iter()
        .map(RankedThing::from_tuple)
        .collect();
    let statistics = Statistics { category, things };
    Ok((StatusCode::OK, Json(statistics)))
}

const DRAW_TWO_RANKED_THINGS_FROM_CATEGORY: &str = "\
    SELECT \
        r.id, \
        r.thing_id, \
        r.category_id, \
        r.score, \
        r.run, \
        t.id, \
        t.name, \
        t.file \
    FROM \
        rank r \
        JOIN thing t ON r.thing_id = t.id \
        JOIN category c ON r.category_id = c.id \
    WHERE \
        r.category_id = $1 \
        AND r.deleted IS NULL \
    ORDER BY \
        r.run, r.shuffle \
    LIMIT 2\
";

pub async fn draw_two_things(state: &State<AppState>, category_id: i32) -> Result<(RankedThing, RankedThing), AppError> {
    
    log::trace!("Drawing two things in category {}", category_id);
    let (thing_a, thing_b) = {
        let thing_rows: Vec<(i32, i32, i32, f64, i32, i32, String, String)> = sqlx::query_as(DRAW_TWO_RANKED_THINGS_FROM_CATEGORY)
            .bind(category_id)
            .fetch_all(&state.pool)
            .await?;
        if thing_rows.len() != 2 {
            return Err(AppError::NotEnoughThings);  
        }
        let mut things_iter = thing_rows.into_iter().map(RankedThing::from_tuple);
        (things_iter.next().unwrap(), things_iter.next().unwrap())
    };

    log::trace!("Discarding things {} and {} for the next run", thing_a.thing.id, thing_b.thing.id);
    let next_run = thing_a.rank.run.max(thing_b.rank.run) + 1;
    sqlx::query("UPDATE rank SET run=$1, shuffle=RANDOM() WHERE thing_id IN ($2,$3) AND category_id=$4")
        .bind(next_run)
        .bind(thing_a.thing.id)
        .bind(thing_b.thing.id)
        .bind(category_id)
        .execute(&state.pool)
        .await?;

    // Done
    Ok((thing_a, thing_b))
}