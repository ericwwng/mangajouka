use axum::extract::Path;
use axum::routing::get;
use axum::{Form, Json, Router};
use axum_error::Result;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

mod mangadex;

#[tokio::main]
async fn main() -> Result<()> {
    let _ = dotenv::dotenv();
    let url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePool::connect(&url).await?;

    let app = Router::new()
        .route("/api/manga/:page", get(manga))
        .route("/api/cover", get(cover))
        // TODO: Look into what this does
        .layer(CorsLayer::very_permissive())
        // TODO: Look into other middleware layers to add
        .with_state(pool);

    // TODO: Change this if we deploy to a real server
    let address = SocketAddr::from(([0, 0, 0, 0], 8000));
    Ok(axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?)
}

// TODO: Potentially change Path to use a Form if we need to read more than just page
async fn manga(Path(page): Path<i32>) -> Result<Json<mangadex::client::MangaList>> {
    // TODO: Read user tag input
    let included_tags = vec!["fantasy"];
    let included_tag_ids =
        mangadex::client::get_tag_ids(&included_tags, mangadex::common::SupportedLanguage::English)
            .await;

    // TODO: Change limit from to 100
    let limit = 2;

    let mangas = mangadex::client::get_manga(&included_tag_ids, limit, limit * page).await;

    Ok(Json(mangas))
}

#[derive(Serialize, Deserialize)]
struct CoverForm {
    manga_id: Uuid,
    filename: String,
}

async fn cover(Form(cover): Form<CoverForm>) -> Result<String> {
    let url = mangadex::client::get_cover_art_url(&cover.manga_id, &cover.filename).await;
    Ok(url)
}
