use dotenvy::dotenv;
use rankit::AppResources;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenv()?;
    env_logger::init();
    let resources = AppResources::from_env().await?;
    Ok(())
}
