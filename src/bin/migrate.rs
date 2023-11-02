use rankit::env;
use refinery::config::Config;


mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("src/bin/migrations");
}

#[tokio::main]
async fn main() {
    if let Err(err) = start().await {
        eprintln!("Migration failed: {err}");
    }
}


async fn start() -> Result<(), anyhow::Error> {
    dotenvy::dotenv()?;
    let mut config = Config::from_env_var(env::APP_DB)?;
    embedded::migrations::runner()
        .run_async(&mut config)
        .await?;
    Ok(())
}