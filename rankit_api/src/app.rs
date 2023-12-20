use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use derive_more::{Deref, DerefMut};
use axum::Json;
use axum::http::StatusCode;
use axum::routing::{Router, post, get, put, delete};
use axum::response::{Response, IntoResponse};
use axum::middleware;
use http::Method;
use tower_http::cors::{CorsLayer, AllowOrigin, AllowHeaders};
use tower_http::services::ServeDir;
use derive_more::{Error, Display, From};
use jsonwebtoken::{EncodingKey, DecodingKey};
use regex::Regex;
use sqlx::{postgres::PgPoolOptions, PgPool};
use crate::account::create_root_account;
use crate::email::{DynEmailService, EmailServiceError};
use crate::{account, thing, env_names, category, rank, migrate};
use crate::file::{DynFileService, FileServiceError};


/// Creates application router.
pub async fn create_app_from_env(migrate: bool) -> Result<Router, anyhow::Error> {

    // Creates shared state and migrates DB if requested.
    let state = AppState::from_env().await?;
    if migrate {
        migrate::migrate().await?;
        create_root_account(
            read_var(env_names::API_ROOT_ACCOUNT_NAME)?,
            read_var(env_names::API_ROOT_ACCOUNT_EMAIL)?,
            read_var(env_names::API_ROOT_ACCOUNT_PASSWORD)?,
            &state.pool
        ).await?;
    }

    // Middleware
    let authenticate    = middleware::from_fn_with_state(state.clone(), account::authenticate);
    let authorize_root  = middleware::from_fn(account::authorize_root);
    let authorize_admin = middleware::from_fn(account::authorize_admin);
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::HEAD, Method::POST, Method::PUT, Method::DELETE, Method::PATCH])
        .allow_headers(AllowHeaders::any())
        .allow_origin(AllowOrigin::predicate(|header_value, _request_parts| {
            println!("{}", header_value.to_str().unwrap());
            true
        }));

    // App
    let mut app = Router::new()
        .route("/api/account/role",             put(account::update_role))  // Updates an account's role.
        // Above routes require root authorization.
        .route_layer(authorize_root)
        .route("/api/thing",                                post(thing::create))        // Creates a new Thing.
        .route("/api/thing/:id",                            delete(thing::delete))      // Deletes a Thing.
        .route("/api/category",                             post(category::create))     // Creates a new Category.
        .route("/api/category/:id",                         delete(category::delete))   // Deletes a Category.
        .route("/api/rank",                                 post(rank::create))         // Creates a new Rank for a Thing in a Category.
        .route("/api/rank/:id",                             delete(rank::delete))       // Deletes a Rank.
        // Above routes require admin or root authorization.
        .route_layer(authorize_admin)           
        .route("/api/account/start_poll",                   put(account::start_poll))   // Puts current account into a "polling state" for a particular category.
        .route("/api/account/end_poll",                     put(account::end_poll))     // Takes current account out of "polling state" by having them submit an answer.
        // Above routes require authentication.         
        .route_layer(authenticate)          
        .route("/api/account",                              post(account::create))      // Creates a new account.
        .route("/api/account/:account_id/verify/:token",    post(account::verify))      // Verifies an account.
        .route("/api/login",                                post(account::login))       // Logs in an account and return a Claims JWT.
        .route("/api/things",                               get(thing::list))           // Gets all Things.
        .route("/api/thing/:id",                            get(thing::single))         // Gets a single Thing.
        .route("/api/categories",                           get(category::list))        // Gets all Categories.
        .route("/api/category/:id",                         get(category::single))      // Gets a single Category.
        .route("/api/category/:id/statistics",              get(category::statistics))  // Gets statistics for a Category.
        .route_layer(cors)
        .with_state(state.clone()); 

    // Configures app based on environment
    if state.typ == AppType::Local {
        app = app.nest_service("/assets", ServeDir::new("assets"));
    }
    Ok(app)
}

/// Type alias for all app responses.
pub type JsonResult<T> = Result<(StatusCode, Json<T>), AppError>;

/// Shared state used by application.
#[derive(Clone, Deref, DerefMut)]
pub struct AppState(Arc<AppStateInner>);

impl AppState {
    pub async fn from_env() -> Result<Self, StartupError> {
        let app_type: AppType = read_var(env_names::API_TYPE)?;
        let (file_store, email_service) = match app_type {
            AppType::Local => (DynFileService::filesystem("assets"), DynEmailService::filesystem("emails")),
            AppType::Aws => return Err(StartupError::AppTypeNotYetSupported(app_type)),
        };
        let pg_str: String                  = read_var(env_names::API_DB)?;
        let claims_duration: u64            = read_var(env_names::API_CLAIMS_DURATION)?;
        let claims_duration                 = Duration::from_secs(claims_duration);
        let claims_secret: String           = read_var(env_names::API_CLAIMS_DURATION)?;
        let claims_secret                   = claims_secret.as_bytes();
        let pool                            = PgPoolOptions::new().max_connections(32).connect(&pg_str).await?;
        let state = AppStateInner {
            file_service: file_store,
            email_service,
            claims_duration,
            claims_encoding_key: EncodingKey::from_secret(claims_secret),
            claims_decoding_key: DecodingKey::from_secret(claims_secret),
            max_image_width: 512,
            max_image_height: 512,
            thing_name_pattern: Regex::new(r"^[a-zA-Z0-9_]+$").unwrap(),
            pool,
            typ: app_type
        };
        Ok(Self(Arc::new(state)))
    }
}

/// Metadata about the location in which the application is hosted.
#[derive(Copy, Clone, Eq, PartialEq, Display, Debug)]
pub enum AppType { Local, Aws }
impl FromStr for AppType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "local" => Ok(Self::Local),
            "aws" => Ok(Self::Aws),
            _ => return Err(())
        }
    }
}

/// Inner value of an [`AppState`].
pub struct AppStateInner {
    pub file_service: DynFileService,
    pub email_service: DynEmailService,
    pub claims_duration: Duration,
    pub claims_encoding_key: EncodingKey,
    pub claims_decoding_key: DecodingKey,
    pub max_image_width: u32,
    pub max_image_height: u32,
    pub thing_name_pattern: Regex,
    pub pool: PgPool,
    pub typ: AppType
}

/// Error that can occur when creating an [`AppState`].
#[derive(Error, Display, Debug, From)]
pub enum StartupError {
    DotenvyError(dotenvy::Error),
    #[from(ignore)]
    #[display(fmt="Missing variable '{_0}'")]
    MissingVar(#[error(not(source))] String),
    #[display(fmt="Failed to parse variable '{_0}'")]
    FailedParsingVar(#[error(not(source))] String),
    #[display(fmt="SQLX Error '{_0}'")]
    SqlxError(sqlx::Error),
    #[display(fmt="AppType {_0} not yet supported")]
    #[from(ignore)]
    AppTypeNotYetSupported(#[error(not(source))] AppType)
}


/// Error that can occur when the application is running.
#[derive(Error, Display, Debug, From)]
pub enum AppError {
    NonAsciiHeader,
    #[from(ignore)]
    MissingMultipartField(#[error(not(source))] &'static str),
    BadThingName,
    CannotModifyRootAccountRole,
    MissingPassword,
    InvalidVerificationToken,
    VerificationTokenNotFound,
    BadRequest,
    MissingEmailOrUsername,
    MissingAuthHeader,
    NoMatchingAccount,
    InvalidAuthToken,
    ClaimsError(jsonwebtoken::errors::Error),
    Unauthenticated,
    Unauthorized,
    CategoryNotFound,
    ThingNotFound,
    RankNotFound,
    AccountNotFound,
    AccountNotVerified,
    DuplicateAccount,
    ThingOrCategoryNotFound,
    DuplicateRecord,
    NotEnoughThings,
    NotInPollingState,
    MultipartError(axum::extract::multipart::MultipartError),
    SqlxError(sqlx::Error),
    PasswordHashError(scrypt::password_hash::Error),
    FileServiceError(FileServiceError),
    EmailServiceError(EmailServiceError),
    #[from(ignore)]
    InternalServerError(#[error(not(source))] &'static str)
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::NonAsciiHeader                => (StatusCode::BAD_REQUEST,    "Header contained non ascii characters").into_response(),
            AppError::MissingMultipartField(field)  => (StatusCode::BAD_REQUEST,    format!("Missing multipart field '{field}'")).into_response(),
            AppError::BadThingName                  => (StatusCode::BAD_REQUEST,    "Bad thing name").into_response(),
            AppError::CannotModifyRootAccountRole   => (StatusCode::BAD_REQUEST,    "Cannot modify role of root account").into_response(),
            AppError::MissingPassword               => (StatusCode::BAD_REQUEST,    "Missing password").into_response(),
            AppError::InvalidVerificationToken      => (StatusCode::BAD_REQUEST,    "Invalid verification token").into_response(),
            AppError::BadRequest                    => (StatusCode::BAD_REQUEST,    "Bad request").into_response(),
            AppError::MissingEmailOrUsername        => (StatusCode::UNAUTHORIZED,   "Missing email or username").into_response(),
            AppError::MissingAuthHeader             => (StatusCode::UNAUTHORIZED,   "Missing auth header").into_response(),
            AppError::NoMatchingAccount             => (StatusCode::UNAUTHORIZED,   "No matching account").into_response(),
            AppError::InvalidAuthToken              => (StatusCode::UNAUTHORIZED,   "Invalid auth token").into_response(),
            AppError::ClaimsError(error)            => (StatusCode::UNAUTHORIZED,   format!("Claims error: {error}")).into_response(),
            AppError::Unauthenticated               => (StatusCode::UNAUTHORIZED,   "Failed to authenticate").into_response(),
            AppError::Unauthorized                  => (StatusCode::UNAUTHORIZED,   "Account lacks privileges").into_response(),
            AppError::CategoryNotFound              => (StatusCode::NOT_FOUND,      "Category not found").into_response(),
            AppError::ThingNotFound                 => (StatusCode::NOT_FOUND,      "Thing not found").into_response(),
            AppError::RankNotFound                  => (StatusCode::NOT_FOUND,      "Rank not found").into_response(),
            AppError::ThingOrCategoryNotFound       => (StatusCode::NOT_FOUND,      "Thing or category not found").into_response(),
            AppError::AccountNotFound               => (StatusCode::NOT_FOUND,      "Account not found").into_response(),
            AppError::VerificationTokenNotFound     => (StatusCode::NOT_FOUND,      "Verification token not found").into_response(),
            AppError::AccountNotVerified            => (StatusCode::CONFLICT,       "Account not verified").into_response(),
            AppError::DuplicateAccount              => (StatusCode::CONFLICT,       "Duplicate account").into_response(),
            AppError::DuplicateRecord               => (StatusCode::CONFLICT,       "Duplicate record").into_response(),
            AppError::NotEnoughThings               => (StatusCode::CONFLICT,       "Not enough things in category to compare").into_response(),
            AppError::NotInPollingState             => (StatusCode::CONFLICT,       "Account not in a polling state").into_response(),

            // 500 (Internal server error)
            AppError::MultipartError(error) => {
                log::error!("Multipart error: {error:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
            AppError::SqlxError(error) => {
                log::error!("SQL Error: {error:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            },
            AppError::PasswordHashError(error) => {
                log::error!("Password Hash Error: {error:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            },
            AppError::FileServiceError(error) => {
                log::error!("File service error: {error:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            },
            AppError::EmailServiceError(error) => {
                log::error!("Email service error: {error:?}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            },
            AppError::InternalServerError(msg) => {
                log::error!("{msg}");
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
            }
        }
    }
}

/// Reads and parses an environment variable.
pub fn read_var<T: FromStr>(var_name: &str) -> Result<T, StartupError> {
    let Ok(value) = dotenvy::var(var_name) else {
        return Err(StartupError::MissingVar(var_name.into()));
    };
    let Ok(value) = value.parse() else {
        return Err(StartupError::FailedParsingVar(var_name.into()));
    };
    Ok(value)
}