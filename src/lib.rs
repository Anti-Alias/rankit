pub mod env;

use derive_more::{Error, Display, From};
use sqlx::{postgres::PgPoolOptions, PgPool};

/// Shared resources used by application.
#[derive(Clone)]
pub struct AppResources {
    pub pool: PgPool
}

impl AppResources {
    pub async fn from_env() -> Result<Self, AppError> {
        log::info!("Connecting to DB");
        let pg_str = read_var("POSTGRES_CONNECTION")?;
        let pool = PgPoolOptions::new()
            .max_connections(32)
            .connect(&pg_str)
            .await?;
        Ok(Self { pool })
    }
}

#[derive(Error, Display, Debug, From)]
pub enum AppError {
    DotenvyError(dotenvy::Error),
    #[from(ignore)]
    MissingEnvironmentVar(#[error(not(source))] String),
    SqlxError(sqlx::Error)
}

pub fn read_var(var_name: &str) -> Result<String, AppError> {
    let Ok(value) = dotenvy::var(var_name) else {
        return Err(AppError::MissingEnvironmentVar(var_name.into()));
    };
    Ok(value)
}