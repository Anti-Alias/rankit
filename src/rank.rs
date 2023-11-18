use axum::{extract::State, Json};
use axum::http::StatusCode;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use tokio::try_join;
use crate::thing;
use crate::app::{AppState, JsonResult, AppError};

const SCORE_INITIAL: i32 = 1200;
const DRAW_TWO_QUERY: &str = "\
    SELECT t.id, t.name, t.file, r.score, r.run \
    FROM \
        rank r ON t.id = r.thing_id \
        JOIN thing t \
        JOIN category c ON r.category_id = c.id \
    WHERE \
        r.category_id = $1 AND \
        t.deleted IS NULL AND \
        c.deleted IS NULL \
    ORDER BY \
        r.run, r.shuffle \
    LIMIT 2\
";

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
    pub thing_id: i32,
    pub category_id: i32,
    pub score: f64,
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

#[derive(Serialize, Deserialize, FromRow, Clone, PartialEq, Debug)]
pub struct RankedThing {
    pub thing: thing::Thing,
    pub score: f64,
    #[serde(skip_serializing)]
    pub run: i32,
}

/// Associates a [`Thing`](crate::thing::Thing) to a [`Category`](crate::category::Category)
/// by inserting a [`Rank`].
/// Gives it an initial score.
pub async fn create(state: State<AppState>, request: Json<CreateRequest>) -> JsonResult<CreateResponse> {

    // Checks for the existance of the specified thing and category.
    log::info!("Checking for existance of thing {} and category {}", request.thing_id, request.category_id);
    let thing_and_cat_exist: (bool,) = sqlx::query_as(THING_AND_CATEGORY_EXIST)
        .bind(request.thing_id)
        .bind(request.category_id)
        .fetch_one(&state.pool)
        .await?;
    if !thing_and_cat_exist.0 {
        return Err(AppError::ThingOrCategoryNotFound);
    }

    // Fetches existing rank state.
    log::info!("Checking for existing rank state");
    let current_run = get_run_of_category(&state, request.category_id);
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
    let rank: Rank = sqlx::query_as("INSERT INTO rank (thing_id, category_id, score, run) VALUES ($1,$2,$3,$4) RETURNING thing_id, category_id, score")
        .bind(request.thing_id)
        .bind(request.category_id)
        .bind(SCORE_INITIAL)
        .bind(current_run)
        .fetch_one(&state.pool)
        .await?;
    Ok((StatusCode::CREATED, Json(CreateResponse { rank })))
}

pub async fn draw_two_things(state: &State<AppState>, category_id: i32) -> Result<(RankedThing, RankedThing), AppError> {
    
    log::info!("Drawing two 'things' in the 'category' {}", category_id);
    let (thing_a, thing_b) = {
        let scored_things: Vec<(i32, String, String, f64, i32)> = sqlx::query_as(DRAW_TWO_QUERY)
            .bind(category_id)
            .fetch_all(&state.pool)
            .await?;
        if scored_things.len() != 2 {
            return Err(AppError::NotEnoughThings);
        }
        let mut things = scored_things.into_iter().map(|row| RankedThing {
            thing: thing::Thing { id: row.0, name: row.1, file: row.2 },
            score: row.3,
            run: row.4
        });
        (things.next().unwrap(), things.next().unwrap())
    };

    log::info!("Discarding things {} and {} for the next run", thing_a.thing.id, thing_b.thing.id);
    let next_run = thing_a.run.max(thing_b.run) + 1;
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

async fn get_run_of_category(state: &State<AppState>, category_id: i32) -> Result<i32, sqlx::Error> {
    let run: (i32,) = sqlx::query_as("SELECT run FROM rank WHERE category_id=$1 ORDER BY run, shuffle LIMIT 1")
        .bind(category_id)
        .fetch_optional(&state.pool)
        .await?
        .unwrap_or((0,));
    Ok(run.0)
}