use axum::{routing::get, Router};
use axum_error::Result;
use sqlx::{Connection, PgConnection};

use crate::{configuration::get_configuration, router::ApiContext};

pub fn router() -> Router<ApiContext> {
    Router::new().route("/health_check", get(health_check))
}

async fn health_check() -> Result<()> {
    let configuration = get_configuration().expect("Failed to read configuration");
    let connection_string = configuration.database.connection_string();
    let _connection = PgConnection::connect(&connection_string)
        .await
        .expect("Failed to connect to Postgres");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn health_check_succeeds() {
        let expected = ();
        let res = health_check().await.unwrap();

        assert_eq!(expected, res);
    }
}
