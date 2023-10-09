use std::collections::HashSet;

use axum::{
    extract::{Path, State},
    routing::{get, post},
    Form, Json, Router,
};
use axum_error::Result;
use mangadex_api::MangaDexClient;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{mangadex, router::ApiContext};

pub(crate) fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/manga/:page", get(manga))
        .route("/api/cover", get(cover))
        .route("/api/filter", post(add_filtered_manga))
        .route("/api/filter", get(get_filtered_mangas))
}

#[derive(Serialize, Deserialize)]
struct CoverForm {
    manga_id: Uuid,
    filename: String,
}

#[derive(Serialize, Deserialize)]
struct MangaForm {
    manga_id: String,
}

#[derive(Serialize, Deserialize)]
struct FilteredManga {
    user_id: i64,
    manga_id: String,
}

// TODO: Potentially change Path to use a Form if we need to read more than just page
async fn manga(
    State(context): State<ApiContext>,
    Path(page): Path<i32>,
) -> Result<Json<mangadex::client::MangaList>> {
    let client = MangaDexClient::default();
    // TODO: Read user tag input
    let included_tags = vec!["fantasy"];
    let included_tag_ids = mangadex::client::get_tag_ids(client, &included_tags)
        .await
        .unwrap();

    let filtered_mangas = sqlx::query!(
        "SELECT user_id, manga_id FROM filtered_mangas WHERE user_id = $1",
        0
    )
    .fetch_all(&context.db)
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

async fn cover(Form(cover): Form<CoverForm>) -> Result<String> {
    //TODO: So that we don't hotlink, we should upload image to a server we own and serve that
    //image instead
    let url = mangadex::client::get_cover_art_url(&cover.manga_id, &cover.filename).await;
    Ok(url)
}

async fn add_filtered_manga(State(context): State<ApiContext>, Json(manga): Json<MangaForm>) {
    sqlx::query!(
        r#"INSERT INTO filtered_mangas (user_id, manga_id) VALUES ($1, $2)"#,
        0,
        manga.manga_id
    )
    .execute(&context.db)
    .await
    .unwrap();
}

// TODO: Take in form for user/password
async fn get_filtered_mangas(State(context): State<ApiContext>) -> Result<Json<Vec<String>>> {
    // TODO: implement user password logic
    let filtered_mangas = sqlx::query_as!(
        FilteredManga,
        "SELECT user_id, manga_id FROM filtered_mangas"
    )
    .fetch_all(&context.db)
    .await?
    .into_iter()
    .map(|manga| manga.manga_id)
    .collect();

    Ok(Json(filtered_mangas))
}
