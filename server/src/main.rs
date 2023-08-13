use axum_error::Result;
use sqlx::SqlitePool;

mod health_check;
mod manga;
mod mangadex;
mod router;

#[tokio::main]
async fn main() -> Result<()> {
    // This will error if .env does not exist, but it is expected to not exist if application is
    // deployed
    dotenv::dotenv().ok();

    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;

    router::serve(pool).await?;

    Ok(())
}
