use axum::extract::{Path, State};
use axum::routing::{get, post};
use axum::{Form, Json, Router};
use axum_error::Result;
use mangadex_api::MangaDexClient;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::collections::HashSet;
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
        .route("/api/filter", post(add_filtered_manga))
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
async fn manga(
    State(pool): State<SqlitePool>,
    Path(page): Path<i32>,
) -> Result<Json<mangadex::client::MangaList>> {
    let client = MangaDexClient::default();
    // TODO: Read user tag input
    let included_tags = vec!["fantasy"];
    let included_tag_ids = mangadex::client::get_tag_ids(client, &included_tags)
        .await
        .unwrap();

    let filtered_mangas = sqlx::query!(
        "SELECT user_id, manga_id FROM filtered_mangas WHERE user_id = ?",
        0
    )
    .fetch_all(&pool)
    .await?;

    let mut filtered_mangas_set = HashSet::new();
    for manga in filtered_mangas {
        filtered_mangas_set.insert(manga.manga_id);
    }

    // TODO: Change limit from to 100
    let limit = 20;

    let mut mangas = mangadex::client::get_manga(&included_tag_ids, limit, limit * page).await;

    mangas
        .data
        .retain(|manga| !filtered_mangas_set.contains(&manga.id.to_string()));

    Ok(Json(mangas))
}

#[derive(Serialize, Deserialize)]
struct CoverForm {
    manga_id: Uuid,
    filename: String,
}

async fn cover(Form(cover): Form<CoverForm>) -> Result<String> {
    //TODO: So that we don't hotlink, we should upload image to a server we own and serve that
    //image instead
    let url = mangadex::client::get_cover_art_url(&cover.manga_id, &cover.filename).await;
    Ok(url)
}

#[derive(Serialize, Deserialize)]
struct MangaForm {
    manga_id: String,
}

async fn add_filtered_manga(State(pool): State<SqlitePool>, Json(manga): Json<MangaForm>) {
    sqlx::query!(
        "INSERT INTO filtered_mangas (user_id, manga_id) VALUES (?, ?)",
        0,
        manga.manga_id
    )
    .execute(&pool)
    .await
    .unwrap();
}
