pub mod env;
pub mod account;
pub mod migrate;

use std::str::FromStr;
use std::time::Duration;

use axum::{routing::{Router, get, post}, response::{IntoResponse, Response}, http::StatusCode, Json};
use derive_more::{Error, Display, From};
use hmac::{digest::{InvalidLength, KeyInit}, Hmac};
use sha2::Sha256;
use sqlx::{postgres::PgPoolOptions, PgPool};

/// Type alias for all app responses.
pub type JsonResult<T> = Result<Json<T>, AppError>;

/// Shared resources used by application.
#[derive(Clone)]
pub struct AppState {
    pub claims_duration: Duration,
    pub claims_key: Hmac<Sha256>,
    pub pool: PgPool,
}

impl AppState {
    pub async fn from_env() -> Result<Self, StartupError> {
        log::info!("Connecting to DB");
        let pg_str: String = read_var(env::APP_DB)?;
        let claims_duration: u64 = read_var(env::APP_CLAIMS_DURATION)?;
        let claims_duration = Duration::from_secs(claims_duration);
        let claims_secret: String = read_var(env::APP_CLAIMS_DURATION)?;
        let pool = PgPoolOptions::new()
            .max_connections(32)
            .connect(&pg_str)
            .await?;
        let claims_key: Hmac<Sha256> = Hmac::new_from_slice(claims_secret.as_bytes())?;
        Ok(Self {
            claims_duration,
            claims_key,
            pool
        })
    }
}

/// Error that can occur when creating an [`AppState`].
#[derive(Error, Display, Debug, From)]
pub enum StartupError {
    DotenvyError(dotenvy::Error),
    #[from(ignore)]
    MissingVar(#[error(not(source))] String),
    FailedParsingVar(#[error(not(source))] String),
    HmacCreationFailed(InvalidLength),
    SqlxError(sqlx::Error)
}


/// Error that can occur when the application is running.
#[derive(Error, Display, Debug, From)]
pub enum AppError {
    DuplicateAccountName,
    DuplicateAccountEmail,
    EmailOrUsernameRequired,
    AuthenticationFailed,
    JwtCreationFailed(jwt::Error),
    SqlxError(sqlx::Error),
    PasswordHashError(scrypt::password_hash::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::DuplicateAccountName      => (StatusCode::BAD_REQUEST,    "Duplicate account name"),
            AppError::DuplicateAccountEmail     => (StatusCode::BAD_REQUEST,    "Duplicate account email"),
            AppError::EmailOrUsernameRequired   => (StatusCode::UNAUTHORIZED,   "Email or username required"),
            AppError::AuthenticationFailed      => (StatusCode::UNAUTHORIZED,   "Authentication failed"),
            AppError::JwtCreationFailed(error) => {
                log::error!("Failed to create JWT: {error}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            },
            AppError::SqlxError(error) => {
                log::error!("SQL Error: {error}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            },
            AppError::PasswordHashError(error) => {
                log::error!("Password Hash Error: {error}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            },
            
        }.into_response()
    }
}

pub fn read_var<T: FromStr>(var_name: &str) -> Result<T, StartupError> {
    let Ok(value) = dotenvy::var(var_name) else {
        return Err(StartupError::MissingVar(var_name.into()));
    };
    let Ok(value) = value.parse() else {
        return Err(StartupError::FailedParsingVar(var_name.into()));
    };
    Ok(value)
}

/// Creates application to serve.
pub async fn app(state: AppState) -> Result<Router, anyhow::Error> {
    let router = Router::new()
        .route("/health", get(health))
        .route("/account", post(account::create_account))
        .route("/account/login", post(account::login))
        .with_state(state);
    Ok(router)
}

 async fn health() -> &'static str {
    "Server is healthy"
 }