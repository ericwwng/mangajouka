use axum_error::Result;
use configuration::get_configuration;
use sqlx::postgres::PgPoolOptions;

mod configuration;
mod health_check;
mod manga;
mod mangadex;
mod router;

#[tokio::main]
async fn main() -> Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to Postgres");

    router::serve(pool, configuration.application_port).await?;

    Ok(())
}
