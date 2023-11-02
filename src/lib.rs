pub mod env;
pub mod account;
pub mod migrate;

use axum::{routing::{Router, get, post}, response::{IntoResponse, Response}, http::StatusCode, Json};
use derive_more::{Error, Display, From};
use sqlx::{postgres::PgPoolOptions, PgPool};

pub type JsonResponse<T> = Result<Json<T>, AppError>;

/// Shared resources used by application.
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool
}

impl AppState {
    pub async fn from_env() -> Result<Self, StartupError> {
        log::info!("Connecting to DB");
        let pg_str = read_var(env::APP_DB)?;
        let pool = PgPoolOptions::new()
            .max_connections(32)
            .connect(&pg_str)
            .await?;
        Ok(Self { pool })
    }
}

/// Error that can occur when creating an [`AppState`].
#[derive(Error, Display, Debug, From)]
pub enum StartupError {
    DotenvyError(dotenvy::Error),
    #[from(ignore)]
    MissingEnvironmentVar(#[error(not(source))] String),
    SqlxError(sqlx::Error)
}


/// Error that can occur when the application is running.
#[derive(Error, Display, Debug, From)]
pub enum AppError {
    DuplicateAccountName,
    DuplicateAccountEmail,
    SqlxError(sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::DuplicateAccountName => (StatusCode::BAD_REQUEST, "Duplicate account name"),
            AppError::DuplicateAccountEmail => (StatusCode::BAD_REQUEST, "Duplicate account email"),
            AppError::SqlxError(error) => {
                if is_unique_violation(&error) {
                    (StatusCode::BAD_REQUEST, "Duplicate data")
                }
                else {
                    log::error!("SQL Error: {error}");
                    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
                }
            },
        }.into_response()
    }
}

pub fn read_var(var_name: &str) -> Result<String, StartupError> {
    let Ok(value) = dotenvy::var(var_name) else {
        return Err(StartupError::MissingEnvironmentVar(var_name.into()));
    };
    Ok(value)
}

/// Creates application to serve.
pub async fn app(state: AppState) -> Result<Router, anyhow::Error> {
    let router = Router::new()
        .route("/health", get(health))
        .route("/account", post(account::create_account))
        .with_state(state);
    Ok(router)
}

 async fn health() -> &'static str {
    "Server is healthy"
 }

 pub fn is_unique_violation(error: &sqlx::Error, ) -> bool {
    match error {
        sqlx::Error::Database(err) => {
            match err.kind() {
                sqlx::error::ErrorKind::UniqueViolation => true,
                _ => false,
            }
        },
        _ => false,
    }
 }