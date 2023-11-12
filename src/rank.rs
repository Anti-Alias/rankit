use axum::{extract::State, Json};
use axum::http::StatusCode;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use tokio::try_join;
use crate::app::{AppState, JsonResult, AppError};
use rand::prelude::*;

const SCORE_INITIAL: i32 = 1200;

/// Associates a [`Thing`](crate::thing::Thing), within a [`Category`](crate::category::Category),
/// and gives it a [`Rank`] within that [`Category`](crate::category::Category).
/// Uses ELO rating system: https://en.wikipedia.org/wiki/Elo_rating_system
#[derive(FromRow, Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct Rank {
    pub thing_id: i32,
    pub category_id: i32,
    pub score: i32,
    pub run: i32,
    pub shuffle: i32
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct CreateRequest {
    pub thing_id: i32,
    pub category_id: i32
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct CreateResponse {
    pub rank: Rank
}

/// Associates a [`Thing`](crate::thing::Thing) to a [`Category`](crate::category::Category)
/// by inserting a [`Rank`].
/// Gives it an initial score.
pub async fn create(state: State<AppState>, request: Json<CreateRequest>) -> JsonResult<CreateResponse> {

    // Fetches existing rank state.
    let current_run = get_current_run(&state, request.category_id);
    let rank_count = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) from rank WHERE thing_id=$1 AND category_id=$2")
        .bind(request.thing_id)
        .bind(request.category_id)
        .fetch_one(&state.pool);
    let (current_run, rank_count) = try_join!(current_run, rank_count)?;

    // Fails on duplicate rank.
    if rank_count.0 != 0 {
        return Err(AppError::DuplicateRecord);
    }

    // Generates and inserts a new rank.
    // "Thing" will be "comparable" in the next "run".
    let shuffle: i32 = thread_rng().gen();
    let rank: Rank = sqlx::query_as("INSERT INTO rank (thing_id, category_id, score, run, shuffle) VALUES ($1,$2,$3,$4,$5) RETURNING thing_id, category_id, score, run, shuffle")
        .bind(request.thing_id)
        .bind(request.category_id)
        .bind(SCORE_INITIAL)
        .bind(current_run + 1)
        .bind(shuffle)
        .fetch_one(&state.pool)
        .await?;
    Ok((StatusCode::CREATED, Json(CreateResponse { rank })))
}

async fn get_current_run(state: &State<AppState>, category_id: i32) -> Result<i32, sqlx::Error> {
    let run: (i32,) = sqlx::query_as("SELECT run FROM rank WHERE category_id=$1 ORDER BY run, shuffle LIMIT 1")
        .bind(category_id)
        .fetch_optional(&state.pool)
        .await?
        .unwrap_or((0,));
    Ok(run.0)
}