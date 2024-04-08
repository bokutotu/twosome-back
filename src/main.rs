use std::{net::SocketAddr, sync::Arc};

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use axum_macros::debug_handler;
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

async fn register_user(pool: &PgPool, name: &str, password: &str) -> Result<(), sqlx::Error> {
    let hashed_password = hash(password, DEFAULT_COST).unwrap();
    let user_id = Uuid::new_v4();

    sqlx::query!(
        r#"
        INSERT INTO users (id, name, password)
        VALUES ($1, $2, $3)
        "#,
        user_id,
        name,
        hashed_password
    )
    .execute(pool)
    .await?;

    Ok(())
}

struct DBAuthenticatedUser {
    user_id: String,
    id: Uuid,
}

async fn authenticate_user(
    pool: &PgPool,
    name: &str,
    password: &str,
) -> Result<Option<DBAuthenticatedUser>, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT id, user_id, password FROM users WHERE name = $1
        "#,
        name
    )
    .fetch_optional(pool)
    .await?;

    match result {
        Some(row) => {
            let stored_password = row.password;
            if verify(password, &stored_password).unwrap_or(false) {
                Ok(Some(DBAuthenticatedUser {
                    user_id: row.user_id,
                    id: row.id,
                }))
            } else {
                Ok(None)
            }
        }
        None => Ok(None),
    }
}

#[derive(Clone)]
struct AppState {
    pool: Arc<PgPool>,
}

#[derive(Debug, Deserialize)]
struct UserRegisterRequest {
    name: String,
    password: String,
}

unsafe impl Send for UserRegisterRequest {}
unsafe impl Sync for UserRegisterRequest {}

#[derive(Debug, Serialize)]
struct UserRegisterResponse {
    success: bool,
    user_id: String,
    id: Uuid,
}

unsafe impl Send for UserRegisterResponse {}
unsafe impl Sync for UserRegisterResponse {}

#[debug_handler]
async fn register(
    State(state): State<AppState>,
    Json(request): axum::Json<UserRegisterRequest>,
) -> Result<Json<UserRegisterResponse>, StatusCode> {
    let name = request.name.clone();
    let password = request.password.clone();

    match register_user(&state.pool, &name, &password).await {
        Ok(_) => {
            let user = authenticate_user(&state.pool, &name, &password)
                .await
                .unwrap()
                .unwrap();
            Ok(Json(UserRegisterResponse {
                success: true,
                user_id: user.user_id,
                id: user.id,
            }))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[tokio::main]
async fn main() {
    let pool = PgPool::connect("postgres://postgres:postgres@localhost/postgres")
        .await
        .unwrap();

    let app_state = AppState {
        pool: Arc::new(pool),
    };

    let router = Router::new()
        .route("/register", post(register))
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
