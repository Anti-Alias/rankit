use axum::{extract::State, Extension};
use axum::Json;
use axum::http::StatusCode;
use serde::{Serialize, Deserialize};
use crate::app::AppError;
use crate::{thing, rank, account};
use crate::{app::{AppState, JsonResult}, category::Category};

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct StartPollRequest {
    pub category_id: i32
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct StartPollResponse {
    pub category: Category,
    pub thing_a: thing::Thing,
    pub thing_b: thing::Thing,
}

pub async fn start(
    state: State<AppState>,
    claims: Extension<account::Claims>,
    request: Json<StartPollRequest>
) -> JsonResult<StartPollResponse> {

    log::info!("Getting category {}", request.category_id);
    let category: Option<Category> = sqlx::query_as("SELECT id, name FROM category WHERE id=$1 AND deleted is NULL")
        .bind(request.category_id)
        .fetch_optional(&state.pool)
        .await?;
    let Some(category) = category else {
        return Err(AppError::CategoryNotFound);
    };

    log::info!("Putting account out of polling state");
    sqlx::query("DELETE FROM poll WHERE account_id=$1")
        .bind(claims.id)
        .execute(&state.pool)
        .await?;

    log::info!("Drawing two random 'things'");
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