use axum::{extract::State, Extension};
use axum::Json;
use axum::http::StatusCode;
use serde::{Serialize, Deserialize};
use sqlx::Acquire;
use crate::{thing, rank, account};
use crate::app::{AppState, AppError, JsonResult};
use crate::category::Category;

const ELO_SCALE: f64 = 32.0;
const ELO_C: f64 = 400.0;
const GET_POLL_DATA: &str = "\
    SELECT \
        p.category_id as category_id, \
        ra.thing_id AS thing_id_a, \
        rb.thing_id AS thing_id_b, \
        ra.score AS thing_score_a, \
        rb.score AS thing_score_b \
    FROM \
        poll p \
        JOIN rank ra ON p.category_id = ra.category_id AND p.thing_id_a = ra.thing_id \
        JOIN rank rb ON p.category_id = rb.category_id AND p.thing_id_b = rb.thing_id \
    WHERE \
        account_id = $1 \
    LIMIT 1\
";
const UPDATE_RANK: &str = "UPDATE rank SET score=$1 WHERE thing_id=$2 AND category_id=$3";

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct StartPollRequest {
    pub category_id: i32,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct StartPollResponse {
    pub category: Category,
    pub thing_a: thing::Thing,
    pub thing_b: thing::Thing,
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct FinishPollRequest {
    pub preference: Preference
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum Preference { A, B }

pub async fn start(state: State<AppState>, claims: Extension<account::Claims>, request: Json<StartPollRequest>) -> JsonResult<StartPollResponse> {

    log::trace!("Getting category {}", request.category_id);
    let category: Option<Category> = sqlx::query_as("SELECT id, name FROM category WHERE id=$1 AND deleted is NULL")
        .bind(request.category_id)
        .fetch_optional(&state.pool)
        .await?;
    let Some(category) = category else {
        return Err(AppError::CategoryNotFound);
    };

    log::trace!("Putting account out of polling state");
    sqlx::query("DELETE FROM poll WHERE account_id=$1")
        .bind(claims.id)
        .execute(&state.pool)
        .await?;

    log::trace!("Drawing two random 'things'");
    let (thing_a, thing_b) = rank::draw_two_things(&state, category.id).await?;
    sqlx::query("INSERT INTO poll (account_id, category_id, thing_id_a, thing_id_b) VALUES ($1,$2,$3,$4)")
        .bind(claims.id)
        .bind(category.id)
        .bind(thing_a.thing.id)
        .bind(thing_b.thing.id)
        .execute(&state.pool)
        .await?;

    // Done
    let response = StartPollResponse {
        category,
        thing_a: thing_a.thing,
        thing_b: thing_b.thing
    };
    Ok((StatusCode::CREATED, Json(response)))
}

pub async fn finish(
    state: State<AppState>,
    claims: Extension<account::Claims>,
    request: Json<FinishPollRequest>
) -> Result<StatusCode, AppError> {

    let mut transaction = state.pool.begin().await?;
    let conn = transaction.acquire().await?;
    
    // Gets scores of things referenced in current polling state.
    log::trace!("Fetching polling data for account {}", claims.id);
    let scores: Option<(i32, i32, i32, f64, f64)> = sqlx::query_as(GET_POLL_DATA)
        .bind(claims.id)
        .fetch_optional(&mut *conn)
        .await?;
    let Some((category_id, thing_id_a, thing_id_b, score_a, score_b)) = scores else {
        return Err(AppError::NotInPollingState);
    };

    // Computes updated elo scores.
    let outcome: f64 = match request.preference { Preference::A => 1.0, Preference::B => 0.0 };
    let new_score_a = compute_elo(score_a, score_b, outcome, ELO_SCALE, ELO_C);
    let new_score_b = compute_elo(score_b, score_a, 1.0 - outcome, ELO_SCALE, ELO_C);
    log::trace!("Updated old scores from {}, {} to {}, {}", score_a, score_b, new_score_a, new_score_b);

    // Updates scores
    log::trace!("Updating scores");
    sqlx::query(UPDATE_RANK)
        .bind(new_score_a)
        .bind(thing_id_a)
        .bind(category_id)
        .execute(&mut *conn)
        .await?;
    sqlx::query(UPDATE_RANK)
        .bind(new_score_b)
        .bind(thing_id_b)
        .bind(category_id)
        .execute(&mut *conn)
        .await?;

    // Takes account out of polling state
    sqlx::query("DELETE FROM poll WHERE account_id=$1")
        .bind(claims.id)
        .execute(conn)
        .await?;
    
    transaction.commit().await?;
    Ok(StatusCode::NO_CONTENT)
}

/// A simple ELO calculation.
/// https://en.wikipedia.org/wiki/Elo_rating_system
/// Followed algo from here: https://stanislav-stankovic.medium.com/elo-rating-system-6196cc59941e
fn compute_elo(score_a: f64, score_b: f64, outcome: f64, scale: f64, c: f64) -> f64 {
    let k = scale;
    let sa = outcome;
    let ra = score_a;
    let rb = score_b;

    let qa: f64 = 10_f64.powf(ra / c);
    let qb: f64 = 10_f64.powf(rb / c);
    let ea = qa / (qa + qb);
    ra + k*(sa - ea)
}
