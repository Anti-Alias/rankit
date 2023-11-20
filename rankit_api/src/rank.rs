use axum::Json;
use axum::extract::{State, Path};
use axum::http::StatusCode;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use tokio::try_join;
use crate::thing;
use crate::app::{AppState, JsonResult, AppError};

const SCORE_INITIAL: i32 = 1200;

/// Used to verify that both a "thing" and a "category" exists.
const THING_AND_CATEGORY_EXIST: &str = "\
    SELECT EVERY(count = 1) FROM ( \
        SELECT COUNT(*) FROM thing WHERE id=$1 AND deleted IS NULL \
        UNION \
        SELECT COUNT(*) FROM category WHERE id=$2 AND deleted is null \
    )\
";

/// Associates a [`Thing`](crate::thing::Thing), within a [`Category`](crate::category::Category),
/// and gives it a [`Rank`] within that [`Category`](crate::category::Category).
/// Uses ELO rating system: https://en.wikipedia.org/wiki/Elo_rating_system
#[derive(FromRow, Serialize, Deserialize, Clone,  PartialEq, Debug)]
pub struct Rank {
    pub id: i32,
    pub thing_id: i32,
    pub category_id: i32,
    pub score: f64,
    #[serde(skip)]
    pub run: i32
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct CreateRequest {
    pub thing_id: i32,
    pub category_id: i32
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct CreateResponse {
    pub rank: Rank
}

/// A [`Thing`](crate::thing::Thing) with associated [`Rank`] data for a given [`Category`](crate::category::Category).
#[derive(Serialize, Deserialize, FromRow, Clone, PartialEq, Debug)]
pub struct RankedThing {
    pub rank: Rank,
    pub thing: thing::Thing
}

impl RankedThing {

    /// Helper method for mapping tuples returned from SQL.
    pub fn from_tuple(row: (i32, i32, i32, f64, i32, i32, String, String)) -> Self {
        Self {
            rank:       Rank { id: row.0, thing_id: row.1, category_id: row.2, score: row.3, run: row.4 },
            thing:      thing::Thing { id: row.5, name: row.6, file: row.7 },
        }
    }
}

/// Associates a [`Thing`](crate::thing::Thing) to a [`Category`](crate::category::Category)
/// by inserting a [`Rank`].
/// Gives it an initial score.
pub async fn create(state: State<AppState>, request: Json<CreateRequest>) -> JsonResult<CreateResponse> {

    log::trace!("Checking for existance of thing {} and category {}", request.thing_id, request.category_id);
    let thing_and_cat_exist: (bool,) = sqlx::query_as(THING_AND_CATEGORY_EXIST)
        .bind(request.thing_id)
        .bind(request.category_id)
        .fetch_one(&state.pool)
        .await?;
    if !thing_and_cat_exist.0 {
        return Err(AppError::ThingOrCategoryNotFound);
    }

    log::trace!("Checking for existing rank state");
    let current_run = get_run_of_category(&state, request.category_id);
    let rank_count = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) from rank WHERE thing_id=$1 AND category_id=$2 AND deleted IS NULL")
        .bind(request.thing_id)
        .bind(request.category_id)
        .fetch_one(&state.pool);
    let (current_run, rank_count) = try_join!(current_run, rank_count)?;
    if rank_count.0 != 0 {
        return Err(AppError::DuplicateRecord);
    }

    log::trace!("Inserting rank for thing {}, and category {}", request.thing_id, request.category_id);
    let rank: Rank = sqlx::query_as("INSERT INTO rank (thing_id, category_id, score, run) VALUES ($1,$2,$3,$4) RETURNING id,thing_id,category_id,score,run")
        .bind(request.thing_id)
        .bind(request.category_id)
        .bind(SCORE_INITIAL)
        .bind(current_run)
        .fetch_one(&state.pool)
        .await?;
    Ok((StatusCode::CREATED, Json(CreateResponse { rank })))
}

pub async fn delete(state: State<AppState>, path: Path<i32>) -> Result<StatusCode, AppError> {
    let rank_id = path.0;

    log::trace!("Checking that rank {rank_id} exists");
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM rank WHERE id=$1 AND deleted IS NULL")
        .bind(rank_id)
        .fetch_one(&state.pool)
        .await?;
    if count.0 == 0 {
        return Err(AppError::RankNotFound);
    }
    log::trace!("Deleting rank {rank_id}");
    sqlx::query("UPDATE rank SET deleted=NOW() WHERE id=$1 AND deleted IS NULL")
        .bind(rank_id)
        .execute(&state.pool)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn get_run_of_category(state: &State<AppState>, category_id: i32) -> Result<i32, sqlx::Error> {
    let run: (i32,) = sqlx::query_as("SELECT run FROM rank WHERE category_id=$1 ORDER BY run, shuffle LIMIT 1")
        .bind(category_id)
        .fetch_optional(&state.pool)
        .await?
        .unwrap_or((0,));
    Ok(run.0)
}