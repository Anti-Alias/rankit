use axum::{Json, extract::State};
use scrypt::{password_hash::{rand_core::OsRng, SaltString, PasswordHasher}, Scrypt};
use sqlx::{PgPool, FromRow};
use serde::{Deserialize, Serialize};
use tokio::try_join;
use crate::{AppResponse, AppState, AppError};

/// Request to create an account.
#[derive(Deserialize, Debug)]
pub struct CreateAccountRequest {
    pub name: String,
    pub email: String,
    pub password: String
}

/// Response for a [`CreateAccountRequest`].
#[derive(Serialize, FromRow)]
pub struct CreateAccountResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
}

pub async fn create_account(
    State(state): State<AppState>,
    Json(request): Json<CreateAccountRequest>
) -> AppResponse<CreateAccountResponse> {
    
    // Checks for duplicates
    let name_count = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM account WHERE name = $1")
        .bind(&request.name)
        .fetch_one(&state.pool);
    let email_count = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM account WHERE email = $1")
        .bind(&request.email)
        .fetch_one(&state.pool);
    let (name_count, email_count) = try_join!(name_count, email_count)?;
    if name_count.0 != 0 {
        return Err(AppError::DuplicateAccountName);
    }
    if email_count.0 != 0 {
        return Err(AppError::DuplicateAccountEmail);
    }

    // Hashes + salts password
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Scrypt
        .hash_password(&request.password.as_bytes(), &salt)?
        .to_string();    

    // Inserts account and returns response
    let response = sqlx::query_as("INSERT INTO account (name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email")
        .bind(request.name)
        .bind(request.email)
        .bind(password_hash)
        .fetch_one(&state.pool)
        .await?;
    Ok(Json(response))
}

/// Creates a root user if it does not already exist.
pub async fn create_root_account(
    name: &str,
    email: &str,
    password: &str,
    pool: &PgPool
) -> Result<(), anyhow::Error> {

    // Quits if root account already exists
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM account WHERE role='root'").fetch_one(pool).await?;
    let root_account_exists = row.0 != 0;
    if root_account_exists {
        return Ok(());
    }

    // Hashes + salts password
    let salt = SaltString::generate(&mut OsRng);
    let password_hash = Scrypt
        .hash_password(password.as_bytes(), &salt)?
        .to_string();  

    // Creates root account
    sqlx::query("INSERT INTO account (name, email, password, role) VALUES ($1, $2, $3, 'root')")
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .execute(pool)
        .await?;
    Ok(())
}