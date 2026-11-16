mod log;

use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = dotenvy::dotenv();

    if let Err(err) = log::init() {
        eprintln!("ERROR: logger failed to initialize: {err}");
    }

    let database_url = std::env::var("DATABASE_URL")?;

    let pool = MySqlPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

    pool.close().await;

    Ok(())
}
