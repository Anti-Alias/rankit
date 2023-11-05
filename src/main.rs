use std::net::SocketAddr;
use dotenvy::dotenv;
use rankit::app::*;
use rankit::migrate::migrate;
use rankit::account::create_root_account;
use rankit::env;


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    dotenv()?;
    env_logger::init();

    // Migrates DB
    migrate().await?;

    // Creates shared state and creates root account if it doesn't exist.
    let state = AppState::from_env().await?;
    create_root_account(
        read_var(env::APP_ROOT_ACCOUNT_NAME)?,
        read_var(env::APP_ROOT_ACCOUNT_EMAIL)?,
        read_var(env::APP_ROOT_ACCOUNT_PASSWORD)?,
        &state.pool
    ).await?;

    // Starts server.
    let address: SocketAddr = read_var(env::SERVER_ADDRESS)?;
    let app = create_app(state).await?;
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
