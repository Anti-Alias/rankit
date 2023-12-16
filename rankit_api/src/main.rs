use std::net::SocketAddr;
use dotenvy::dotenv;
use rankit::app::*;
use rankit::env_names;


#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {

    dotenv()?;
    env_logger::init();

    // Starts app.
    let address: SocketAddr = read_var(env_names::API_SERVER_ADDRESS)?;
    let app = create_app_from_env(true).await?;
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
