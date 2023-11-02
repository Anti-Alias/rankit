use std::net::SocketAddr;

use dotenvy::dotenv;
use rankit::{app, read_var, env};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv()?;
    env_logger::init();
    let address: SocketAddr = read_var(env::SERVER_ADDRESS)?.parse()?;
    let app = app().await?;
    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
