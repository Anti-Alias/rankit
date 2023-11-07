use std::io::Cursor;
use axum::{extract::{State, Multipart, Extension}, body::Bytes, Json};
use serde::{Serialize, Deserialize};
use image::{io::Reader as ImageReader, ImageError, GenericImageView, imageops::FilterType, codecs::jpeg::JpegEncoder};
use sqlx::{FromRow, Acquire};
use crate::{AppState, JsonResult, account::Claims, AppError};

/// Request to create a "thing".
#[derive(Deserialize, Clone, Eq, PartialEq, Debug)]
pub struct CreateThingRequest {
    pub name: String,
}

/// Response to a [`CreateThingRequest`].
#[derive(Serialize, FromRow, Clone, Eq, PartialEq, Debug)]
pub struct CreateThingResponse {
    pub id: i32,
    pub name: String,
}

pub async fn create(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    mut multipart: Multipart
) -> JsonResult<CreateThingResponse> {

    // Separates "image" and "request" parts.
    let mut image_bytes: Option<Bytes> = None;
    let mut request_bytes: Option<String> = None;
    while let Some(field) = multipart.next_field().await? {
        match field.name() {
            Some("image")   => image_bytes = Some(field.bytes().await?),
            Some("request") => request_bytes = Some(field.text().await?),
            _ => {},
        }
    }
    let Some(image_bytes) = image_bytes else {
        return Err(AppError::MissingMultipartField("image"));
    };
    let Some(request_str) = request_bytes else {
        return Err(AppError::MissingMultipartField("request"));
    };

    // Parses "image" and "request" parts.
    let jpeg_bytes = match encode_jpeg(&image_bytes, state.max_image_width, state.max_image_height) {
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

    // Checks for duplicate thing.
    let thing_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM thing WHERE name=$1 and account_id=$2")
        .bind(&request.name)
        .bind(claims.id)
        .fetch_one(&state.pool)
        .await?;
    if thing_count.0 != 0 {
        return Err(AppError::DuplicateThing);
    }

    // Saves image entry to DB.
    let file_dir = format!("accounts/{}/things/", claims.id);
    let file_name = format!("{}.jpg", request.name);
    let mut tx = state.pool.begin().await?;
    let conn = tx.acquire().await?;
    let response: CreateThingResponse = sqlx::query_as("INSERT INTO thing (account_id, name, file) VALUES ($1, $2, $3) RETURNING id, name")
        .bind(claims.id)
        .bind(request.name)
        .bind(&file_name)
        .fetch_one(conn)
        .await?;

    // Saves image contents to file store.
    state.file_store.create(Some(&file_dir), &file_name, &jpeg_bytes).await?;
    tx.commit().await?;

    // Done
    Ok(Json(response))
}

// Reads in image bytes, resizes it if too large,
// then output it as jpeg bytes.
fn encode_jpeg(image_bytes: &[u8], max_width: u32, max_height: u32) -> Result<Vec<u8>, ImageError> {
    let cursor = Cursor::new(image_bytes);
    let reader = ImageReader::new(cursor).with_guessed_format()?;
    let mut image = reader.decode()?;
    let (img_width, img_height) = image.dimensions();
    if img_width > max_width || img_height > max_height {
        image = image.resize(max_width, max_height, FilterType::Gaussian);
    }
    let mut jpeg_bytes: Vec<u8> = Vec::new();
    let encoder = JpegEncoder::new(&mut jpeg_bytes);
    image.write_with_encoder(encoder)?;
    Ok(jpeg_bytes)
}