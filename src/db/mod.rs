pub mod models;
pub mod queries;

use sqlx::{sqlite::SqlitePool, Pool, Sqlite};

pub type DbPool = Pool<Sqlite>;

/// Initialize SQLite connection pool
pub async fn init_pool() -> anyhow::Result<DbPool> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:salvo.db".to_string());

    tracing::info!("Connecting to database: {}", database_url);

    let pool = SqlitePool::connect(&database_url).await?;

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    tracing::info!("Database migrations complete");

    Ok(pool)
}
