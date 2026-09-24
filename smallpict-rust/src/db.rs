use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn init_db_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS public.images (
            id VARCHAR(50) PRIMARY KEY,
            filename VARCHAR(255),
            url TEXT,
            size BIGINT,
            size_origin BIGINT,
            mime_type VARCHAR(100),
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );
        "#
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

pub async fn check_db_health(pool: &PgPool) -> bool {
    sqlx::query("SELECT 1").execute(pool).await.is_ok()
}
