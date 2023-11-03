use jwt::SignWithKey;
use axum::{Json, extract::State};
use std::time::Duration;
use chrono::Utc;
use scrypt::{password_hash::{rand_core::OsRng, SaltString, PasswordHasher, PasswordHash, PasswordVerifier}, Scrypt};
use sqlx::{PgPool, FromRow};
use serde::{Deserialize, Serialize};
use tokio::{task, try_join};
use crate::{JsonResult, AppState, AppError};

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

/// Request to login to an account.
#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: String
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
    pub exp: i64,
    pub name: String,
    pub email: String,
    pub role: Role,
}

#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum Role { Basic, Root }

impl Claims {
    pub fn new(name: String, email: String, role: Role, exp_duration: Duration) -> Self {
        let exp = (Utc::now() + exp_duration).timestamp();
        Self {
            exp,
            name,
            email,
            role,
        }
    }
}

pub async fn create_account(
    State(state): State<AppState>,
    Json(request): Json<CreateAccountRequest>,
) -> JsonResult<CreateAccountResponse> {
    
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
    let password_hash = generate_password_hash(request.password).await?;

    // Inserts account and returns response
    let response = sqlx::query_as("INSERT INTO account (name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email")
        .bind(request.name)
        .bind(request.email)
        .bind(password_hash)
        .fetch_one(&state.pool)
        .await?;
    Ok(Json(response))
}

pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<String, AppError> {
    
    // Fetches details of user matching either the email or username.
    let result: Option<(String, String, Role, String)> = match (request.email, request.name) {
        (_, Some(name))     => sqlx::query_as("SELECT email, name, role, password FROM account WHERE name=$1").bind(name).fetch_optional(&state.pool).await?,
        (Some(email), _)    => sqlx::query_as("SELECT email, name, role, password FROM account WHERE email=$1").bind(email).fetch_optional(&state.pool).await?,
        (None, None)        => return Err(AppError::EmailOrUsernameRequired),
    };
    let Some((email, name, role, password)) = result else {
        return Err(AppError::AuthenticationFailed);
    };

    // Checks that passwords match.
    match verify_password(request.password.clone(), password).await {
        Err(scrypt::password_hash::Error::Password) => return Err(AppError::AuthenticationFailed),
        Err(err) => return Err(AppError::PasswordHashError(err)),
        Ok(_) => {},
    }
    
    // Generates a JWT string
    let claims = Claims::new(email, name, role, state.claims_duration);
    let claims_str = claims.sign_with_key(&state.claims_key)?;
    Ok(claims_str)
}

/// Creates a root user if it does not already exist.
pub async fn create_root_account(
    name: String,
    email: String,
    password: String,
    pool: &PgPool
) -> Result<(), anyhow::Error> {

    // Quits if root account already exists
    let row: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM account WHERE role='root'").fetch_one(pool).await?;
    let root_account_exists = row.0 != 0;
    if root_account_exists {
        return Ok(());
    }

    // Hashes + salts password
    let password_hash = generate_password_hash(password.into()).await?;

    // Creates root account
    sqlx::query("INSERT INTO account (name, email, password, role) VALUES ($1, $2, $3, 'root')")
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .execute(pool)
        .await?;
    Ok(())
}

async fn generate_password_hash(password: String) -> Result<String, AppError> {
    task::spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        Scrypt
            .hash_password(password.as_bytes(), &salt)
            .map(|pass_hash| pass_hash.to_string())
            .map_err(|err| AppError::from(err))
    })
    .await
    .expect("generate_password_hash background thread failed to join")
}

async fn verify_password(password: String, existing_password: String) -> scrypt::password_hash::Result<()> {
    task::spawn_blocking(move || {
        let password = password.as_bytes();
        let hash = PasswordHash::new(&existing_password)?;
        Scrypt.verify_password(&password, &hash)
    })
    .await
    .expect("verify_password background thread failed to join")
}