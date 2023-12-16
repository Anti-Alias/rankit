use crate::env_names;
use refinery::config::Config;


mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("src/migrations");
}

pub async fn migrate() -> Result<(), anyhow::Error> {
    dotenvy::dotenv()?;
    let mut config = Config::from_env_var(env_names::APP_DB)?;
    embedded::migrations::runner()
        .run_async(&mut config)
        .await?;
    Ok(())
}