mod log;
mod models;

use std::time::Duration;

use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    if let Err(err) = log::init() {
        eprintln!("ERROR: logger failed to initialize: {err}");
    }

    let database_url = std::env::var("DATABASE_URL")?;

    let pool = MySqlPoolOptions::new()
        .max_connections(50)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(10))
        .connect(&database_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    pool.close().await;

    Ok(())
}
