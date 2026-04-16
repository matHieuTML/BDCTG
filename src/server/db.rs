use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn init_pool() -> Result<PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set (see .env.example)");

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&database_url)
        .await
}

pub async fn db_now(pool: &PgPool) -> Result<String, sqlx::Error> {
    let row: (chrono::DateTime<chrono::Utc>,) =
        sqlx::query_as("SELECT NOW()").fetch_one(pool).await?;

    sqlx::query("INSERT INTO health_check DEFAULT VALUES")
        .execute(pool)
        .await?;

    Ok(row.0.to_rfc3339())
}
