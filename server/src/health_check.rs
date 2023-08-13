use axum::{routing::get, Router};
use axum_error::Result;

use crate::router::ApiContext;

pub fn router() -> Router<ApiContext> {
    Router::new().route("/health_check", get(health_check))
}

async fn health_check() -> Result<()> {
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
