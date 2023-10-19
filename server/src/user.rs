use argon2::{password_hash::SaltString, Argon2, PasswordHash};
use axum::{
    extract::State,
    routing::{get, post},
    Form, Json, Router,
};
use axum_error::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::router::ApiContext;

pub fn router() -> Router<ApiContext> {
    Router::new()
        .route("/api/user", get(user_list))
        .route("/api/user/register", post(register_user))
        .route("/api/user/login", get(login))
}

#[derive(Deserialize, Serialize)]
struct User {
    id: Uuid,
    username: String,
    password_hash: String,
}

#[derive(Deserialize, Serialize)]
struct UserBody {
    id: Uuid,
    username: String,
}

#[derive(Deserialize, Serialize)]
struct RegisterUserForm {
    username: String,
    password: String,
    email: String,
}

#[derive(Deserialize, Serialize)]
struct LoginUserForm {
    username: String,
    password: String,
}

async fn user_list(State(context): State<ApiContext>) -> Result<Json<Vec<User>>> {
    let users = sqlx::query_as!(User, "SELECT id, username, password_hash FROM users")
        .fetch_all(&context.db)
        .await?;

    Ok(Json(users))
}

async fn register_user(
    State(context): State<ApiContext>,
    Json(register_user_form): Json<RegisterUserForm>,
) {
    sqlx::query!(
        r#"INSERT INTO users (id, username, password_hash, email) VALUES ($1, $2, $3, $4)"#,
        Uuid::new_v4(),
        register_user_form.username,
        hash_password(register_user_form.password).await,
        register_user_form.email
    )
    .execute(&context.db)
    .await
    .unwrap();
}

async fn login(
    State(context): State<ApiContext>,
    Form(login_user): Form<LoginUserForm>,
) -> Result<Json<UserBody>> {
    let user = sqlx::query_as!(
        User,
        "SELECT id, username, password_hash FROM users where username = $1",
        login_user.username
    )
    .fetch_optional(&context.db)
    .await?
    .ok_or_else(|| sqlx::Error::RowNotFound)?;

    Ok(Json(UserBody {
        id: user.id,
        username: user.username,
    }))
}

async fn hash_password(password: String) -> String {
    //TODO: Maybe wrap around a tokio::task::spawn_blocking call
    let salt = SaltString::generate(rand::thread_rng());

    PasswordHash::generate(Argon2::default(), password, &salt)
        .unwrap()
        .to_string()
}
