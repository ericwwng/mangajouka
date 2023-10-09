use crate::{health_check, manga};
use axum::Router;
use axum_error::Result;
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
pub struct ApiContext {
    pub db: PgPool,
}

pub async fn serve(db: PgPool, application_port: u16) -> Result<()> {
    let api_context = ApiContext { db };

    let app = api_router(api_context);

    // TODO: Change this if we deploy to a real server
    let address = SocketAddr::from(([0, 0, 0, 0], application_port));
    Ok(axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}

fn api_router(api_context: ApiContext) -> Router {
    Router::new()
        .merge(health_check::router())
        .merge(manga::router())
        // TODO: Look into what this does
        .layer(CorsLayer::very_permissive())
        // TODO: Look into other middleware layers to add
        .with_state(api_context)
}
