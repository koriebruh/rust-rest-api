use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::env;
use dotenv::dotenv;
// use dotenv::from_path;


pub async fn get_pool() -> Result<MySqlPool, sqlx::Error> {
    dotenv().ok();
    // from_path(".env").expect("failed loaded env");

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL measure ready in .env");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(3))
        .idle_timeout(std::time::Duration::from_secs(8 * 60))
        .max_lifetime(std::time::Duration::from_secs(8 * 60))
        .connect(&database_url)
        .await?;

    Ok(pool)
}