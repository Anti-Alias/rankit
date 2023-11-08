use std::io::Cursor;
use axum::{extract::{State, Multipart, Extension, Path}, body::Bytes, Json};
use serde::{Serialize, Deserialize};
use image::{io::Reader as ImageReader, ImageError, GenericImageView, imageops::FilterType, codecs::jpeg::JpegEncoder};
use sqlx::{FromRow, Acquire};
use crate::{AppState, JsonResult, account::Claims, AppError};

/// Request to create a "thing".
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct CreateThingRequest {
    pub name: String,
}

/// Response to a [`CreateThingRequest`].
#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct CreateThingResponse {
    pub thing: Thing
}

#[derive(Serialize, Deserialize, FromRow, Clone, Eq, PartialEq, Debug)]
pub struct Thing {
    pub id: i32,
    pub name: String,
    pub file: String,
}

pub async fn create(state: State<AppState>, claims: Extension<Claims>, mut multipart: Multipart) -> JsonResult<CreateThingResponse> {

    // Separates "image" and "request" parts.
    let mut image_bytes: Option<Bytes> = None;
    let mut request_str: Option<String> = None;
    while let Some(field) = multipart.next_field().await? {
        match field.name() {
            Some("image")   => image_bytes = Some(field.bytes().await?),
            Some("request") => request_str = Some(field.text().await?),
            _ => {},
        }
    }
    let Some(image_bytes) = image_bytes else {
        return Err(AppError::MissingMultipartField("image"));
    };
    let Some(request_str) = request_str else {
        return Err(AppError::MissingMultipartField("request"));
    };

    // Parses "image" and "request" parts.
    let jpeg_bytes = match to_jpeg_bytes(image_bytes, state.max_image_width, state.max_image_height).await {
        Ok(value) => value,
        Err(err) => {
            log::error!("Failed to parse image: {err}");
            return Err(AppError::BadRequest);
        },
    };
    let request: CreateThingRequest = match serde_json::from_str(&request_str) {
        Ok(value) => value,
        Err(err) => {
            log::error!("Failed to parse request json: {err}");
            return Err(AppError::BadRequest);
        },
    };
    if !state.thing_name_pattern.is_match(&request.name) {
        return Err(AppError::BadThingName);
    }

    // Checks for duplicate thing.
    let thing_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM thing WHERE name=$1 and account_id=$2")
        .bind(&request.name)
        .bind(claims.id)
        .fetch_one(&state.pool)
        .await?;
    if thing_count.0 != 0 {
        return Err(AppError::DuplicateThing);
    }

    let file_name = format!("{}.jpg", &request.name);
    let file_path = format!("account/{}/thing/{}", claims.id, &file_name);

    // Saves image to DB and file store.
    let mut tx = state.pool.begin().await?;
    let conn = tx.acquire().await?;
    let thing: Thing = sqlx::query_as("INSERT INTO thing (account_id, name, file) VALUES ($1, $2, $3) RETURNING id, name, file")
        .bind(claims.id)
        .bind(request.name)
        .bind(&file_path)
        .fetch_one(conn)
        .await?;
    state.file_store.create(&file_path, &jpeg_bytes).await?;
    tx.commit().await?;

    // Done
    let response = CreateThingResponse { thing };
    Ok(Json(response))
}

pub async fn single(state: State<AppState>, path: Path<i32>) -> JsonResult<Thing> {
    let thing: Option<Thing> = sqlx::query_as("SELECT id, account_id, name, file FROM thing WHERE id = $1")
        .bind(path.0)
        .fetch_optional(&state.pool)
        .await?;
    let Some(thing) = thing else {
        return Err(AppError::RecordNotFound);
    };
    Ok(Json(thing))
}

/// Lists all things under this account.
pub async fn list(state: State<AppState>, claims: Extension<Claims>) -> JsonResult<Vec<Thing>> {
    let things: Vec<Thing> = sqlx::query_as("SELECT id, account_id, name, file FROM thing WHERE account_id = $1")
        .bind(claims.id)
        .fetch_all(&state.pool)
        .await?;
    Ok(Json(things))
}

// Reads in image bytes, resizes it if too large,
// then output it as jpeg bytes.
async fn to_jpeg_bytes(image_bytes: Bytes, max_width: u32, max_height: u32) -> Result<Vec<u8>, ImageError> {
    tokio::task::spawn_blocking(move || {
        let cursor = Cursor::new(image_bytes);
        let reader = ImageReader::new(cursor).with_guessed_format()?;
        let mut image = reader.decode()?;
        let (img_width, img_height) = image.dimensions();
        if img_width > max_width || img_height > max_height {
            image = image.resize(max_width, max_height, FilterType::Gaussian);
        }
        let mut jpeg_bytes: Vec<u8> = Vec::with_capacity(1024*32);
        let encoder = JpegEncoder::new(&mut jpeg_bytes);
        image.write_with_encoder(encoder)?;
        Ok(jpeg_bytes)
    })
    .await
    .unwrap()
}