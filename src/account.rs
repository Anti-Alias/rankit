use jsonwebtoken::{encode, decode, Header, Validation};
use axum::{Json, extract::State, middleware::Next};
use axum::response::{Response, IntoResponse};
use axum::http::{header, Request};
use reqwest::StatusCode;
use std::time::Duration;
use chrono::Utc;
use scrypt::{password_hash::{rand_core::OsRng, SaltString, PasswordHasher, PasswordHash, PasswordVerifier}, Scrypt};
use sqlx::{PgPool, FromRow};
use serde::{Deserialize, Serialize};
use tokio::task;
use crate::{JsonResult, AppState, AppError};


/// Account information to be encoded as a JWT.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
    /// Expiration time of claims in UTC seconds since epoch.
    pub exp: i64,
    /// Account id.
    pub id: i32,
    /// Account name.
    pub name: String,
    /// Account email.
    pub email: String,
    /// Account role.
    pub role: Role,
}

impl Claims {
    pub fn new(id: i32, name: String, email: String, role: Role, exp_duration: Duration) -> Self {
        let exp = (Utc::now() + exp_duration).timestamp();
        Self { exp, id, name, email, role, }
    }
}

/// Role of an account, determining privileges.
#[derive(Serialize, Deserialize, Copy, Clone, Eq, PartialEq, Debug, sqlx::Type)]
#[sqlx(rename_all = "lowercase")]
pub enum Role { Basic, Root }

/// Request to create a new account.
#[derive(Deserialize, Debug)]
pub struct CreateAccountRequest {
    pub name: String,
    pub email: String,
    pub password: String
}

/// Response to a [`CreateAccountRequest`].
#[derive(Serialize, FromRow)]
pub struct CreateAccountResponse {
    pub id: i32,
    pub name: String,
    pub email: String,
}

/// Request to login to an existing account.
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: String
}

/// Route that creates an account.
pub async fn create(state: State<AppState>, request: Json<CreateAccountRequest>) -> JsonResult<CreateAccountResponse> {
    let state = state.0;
    let request = request.0;
    
    // Checks for duplicate accounts.
    let account_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM account WHERE email=$1 OR name=$2")
        .bind(&request.email)
        .bind(&request.name)
        .fetch_one(&state.pool)
        .await?;
    if account_count.0 != 0 {
        return Err(AppError::DuplicateRecord);
    }

    // Inserts new account and returns response.
    let password_hash = generate_password_hash(request.password).await?;
    let resp_body = sqlx::query_as("INSERT INTO account (name, email, password) VALUES ($1, $2, $3) RETURNING id, name, email")
        .bind(&request.name)
        .bind(&request.email)
        .bind(password_hash)
        .fetch_one(&state.pool)
        .await?;
    let resp_body = Json(resp_body);
    Ok((StatusCode::CREATED, resp_body))
}

/// Route that logs in a user.
pub async fn login(state: State<AppState>, request: Json<LoginRequest>) -> Result<String, AppError> {
    
    // Fetches details of user matching either the email or username.
    let result: Option<(i32, String, String, Role, String)> = match (&request.email, &request.name) {
        (_, Some(name))     => sqlx::query_as("SELECT id, email, name, role, password FROM account WHERE name=$1").bind(name).fetch_optional(&state.pool).await?,
        (Some(email), _)    => sqlx::query_as("SELECT id, email, name, role, password FROM account WHERE email=$1").bind(email).fetch_optional(&state.pool).await?,
        (None, None)        => return Err(AppError::MissingEmailOrUsername),
    };
    let Some((id, email, name, role, password)) = result else {
        return Err(AppError::NoMatchingUser);
    };

    // Checks that passwords match.
    match verify_password(request.password.clone(), password).await {
        Err(scrypt::password_hash::Error::Password) => return Err(AppError::NoMatchingUser),
        Err(err) => return Err(AppError::PasswordHashError(err)),
        Ok(_) => {},
    }
    
    // Generates a JWT string
    let claims = Claims::new(id, email, name, role, state.claims_duration);
    let claims_str = encode(&Header::default(), &claims, &state.claims_encoding_key)?;
    Ok(claims_str)
}

/// Utility function that creates a root account if it does not yet exist.
pub async fn create_root_account(name: String, email: String, password: String, pool: &PgPool) -> Result<StatusCode, anyhow::Error> {

    // Quits if root account already exists.
    let root_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM account WHERE role='root'").fetch_one(pool).await?;
    if root_count.0 != 0 {
        return Ok(StatusCode::NO_CONTENT);
    }

    // Creates root account.
    let password_hash = generate_password_hash(password).await?;
    sqlx::query("INSERT INTO account (name, email, password, role) VALUES ($1, $2, $3, 'root')")
        .bind(name)
        .bind(email)
        .bind(password_hash)
        .execute(pool)
        .await?;
    Ok(StatusCode::CREATED)
}

/// Middleware function that authenticates users.
/// Attaches a [`Claims`] extension.
pub async fn authenticate<B>(state: State<AppState>, mut request: Request<B>, next: Next<B>) -> Response {
    let Some(authorization) = request.headers().get(header::AUTHORIZATION) else {
        return AppError::MissingAuthHeader.into_response();
    };
    let authorization = match authorization.to_str() {
        Ok(value) => value,
        Err(_) => return AppError::NonAsciiHeader.into_response(),
    };
    let authorization = match skip_bearer(authorization) {
        Ok(value) => value,
        Err(err) => return err.into_response(),
    };
    let claims = match decode::<Claims>(authorization, &state.claims_decoding_key, &Validation::default()) {
        Ok(token_data) => token_data.claims,
        Err(err) => return AppError::ClaimsError(err).into_response()
    };
    request.extensions_mut().insert(claims);
    next.run(request).await
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

fn skip_bearer(auth_token: &str) -> Result<&str, AppError> {
    if !auth_token.starts_with("Bearer ") {
        return Err(AppError::Unauthorized);
    }
    Ok(auth_token[7..].trim())
}