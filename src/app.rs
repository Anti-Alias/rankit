use std::str::FromStr;
use std::time::Duration;
use axum::Json;
use axum::http::StatusCode;
use axum::routing::{Router, post};
use axum::response::{Response, IntoResponse};
use axum::middleware::from_fn_with_state;
use derive_more::{Error, Display, From};
use jsonwebtoken::{EncodingKey, DecodingKey};
use sqlx::{postgres::PgPoolOptions, PgPool};
use crate::{account, env};

/// Type alias for all app responses.
pub type JsonResult<T> = Result<Json<T>, AppError>;

/// Shared resources used by application.
#[derive(Clone)]
pub struct AppState {
    pub claims_duration: Duration,
    pub claims_encoding_key: EncodingKey,
    pub claims_decoding_key: DecodingKey,
    pub pool: PgPool,
}

impl AppState {
    pub async fn from_env() -> Result<Self, StartupError> {
        log::info!("Connecting to DB");
        let pg_str: String = read_var(env::APP_DB)?;
        let claims_duration: u64 = read_var(env::APP_CLAIMS_DURATION)?;
        let claims_duration = Duration::from_secs(claims_duration);
        let claims_secret: String = read_var(env::APP_CLAIMS_DURATION)?;
        let claims_secret = claims_secret.as_bytes();
        let pool = PgPoolOptions::new()
            .max_connections(32)
            .connect(&pg_str)
            .await?;
        Ok(Self {
            claims_duration,
            claims_encoding_key: EncodingKey::from_secret(claims_secret),
            claims_decoding_key: DecodingKey::from_secret(claims_secret),
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
    SqlxError(sqlx::Error)
}


/// Error that can occur when the application is running.
#[derive(Error, Display, Debug, From)]
pub enum AppError {
    DuplicateAccountName,
    DuplicateAccountEmail,
    EmailOrUsernameRequired,
    NoMatchingUser,
    ClaimsError(jsonwebtoken::errors::Error),
    Unauthorized,
    MissingAuthHeader,
    InvalidAuthToken,
    NonAsciiHeader,
    SqlxError(sqlx::Error),
    PasswordHashError(scrypt::password_hash::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let response: Response = match self {
            AppError::DuplicateAccountName      => (StatusCode::BAD_REQUEST,    "Duplicate account name").into_response(),
            AppError::DuplicateAccountEmail     => (StatusCode::BAD_REQUEST,    "Duplicate account email").into_response(),
            AppError::NonAsciiHeader            => (StatusCode::BAD_REQUEST,    "Header contained non ascii characters").into_response(),
            AppError::EmailOrUsernameRequired   => (StatusCode::UNAUTHORIZED,   "Email or username required").into_response(),
            AppError::NoMatchingUser            => (StatusCode::UNAUTHORIZED,   "No matching user").into_response(),
            AppError::Unauthorized              => (StatusCode::UNAUTHORIZED,   "Unauthorized").into_response(),
            AppError::MissingAuthHeader         => (StatusCode::UNAUTHORIZED,   "Missing auth header").into_response(),
            AppError::InvalidAuthToken          => (StatusCode::UNAUTHORIZED,   "Invalid auth token").into_response(),
            AppError::ClaimsError(error)        => (StatusCode::UNAUTHORIZED,   format!("Claims error: {error}")).into_response(),
            AppError::SqlxError(error) => {
                log::error!("SQL Error: {error}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            },
            AppError::PasswordHashError(error) => {
                log::error!("Password Hash Error: {error}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            },
            
        };
        response
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

/// Creates application router.
pub async fn create_app(state: AppState) -> Result<Router, anyhow::Error> {
    let authenticate = from_fn_with_state(state.clone(), account::authenticate);
    let router = Router::new()
        .layer(authenticate)
        .route("/account", post(account::create))
        .route("/account/login", post(account::login))
        .with_state(state);
    Ok(router)
}