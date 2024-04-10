use std::{fs, net::SocketAddr, path::Path, sync::Arc};

use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use axum_macros::debug_handler;
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, Clone)]
struct User {
    id: Uuid,
    name: String,
    user_id: String,
}
impl User {
    async fn register(pool: &PgPool, request: &UserRegisterRequest) -> Result<Self, sqlx::Error> {
        let hashed_password =
            hash(&request.password, DEFAULT_COST).map_err(|_| sqlx::Error::PoolTimedOut)?;
        let id = Uuid::new_v4();

        sqlx::query!(
            r#"
            INSERT INTO users (id, name, password, user_id)
            VALUES ($1, $2, $3, $4)
            "#,
            id,
            request.name,
            hashed_password,
            request.user_id
        )
        .execute(pool)
        .await?;

        info!(
            "New user registered: name={}, id={}, user_id={}",
            request.name, id, request.user_id
        );

        Ok(User {
            id,
            name: request.name.to_string(),
            user_id: request.user_id.to_string(),
        })
    }
}

impl User {
    async fn authenticate(
        pool: &PgPool,
        request: &UserLoginRequest,
    ) -> Result<Option<Self>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT id, user_id, name, password FROM users WHERE user_id = $1
            "#,
            request.user_id
        )
        .fetch_optional(pool)
        .await?;

        match result {
            Some(row) => {
                let stored_password = row.password;
                if verify(&request.password, &stored_password).unwrap_or(false) {
                    info!(
                        "User authenticated: user_id={} id={} name={}",
                        row.user_id, row.id, row.name
                    );
                    Ok(Some(User {
                        id: row.id,
                        name: row.name,
                        user_id: row.user_id,
                    }))
                } else {
                    info!(
                        "Authentication failed for user: user_id={}",
                        request.user_id
                    );
                    Ok(None)
                }
            }
            None => {
                info!("User not found: user_id={}", request.user_id);
                Ok(None)
            }
        }
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
    user_id: String,
}

#[derive(Debug, Serialize)]
struct UserRegisterResponse {
    id: Uuid,
}

impl From<User> for UserRegisterResponse {
    fn from(user: User) -> Self {
        UserRegisterResponse { id: user.id }
    }
}

#[debug_handler]
async fn register(
    State(state): State<AppState>,
    Json(request): Json<UserRegisterRequest>,
) -> Result<Json<UserRegisterResponse>, StatusCode> {
    match User::register(&state.pool, &request).await {
        Ok(user) => {
            info!("Registration successful: id={}", user.id);
            Ok(Json(user.into()))
        }
        Err(e) => {
            error!(
                "Registration failed: user_id={}, error={}",
                request.user_id, e
            );
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug, Deserialize)]
struct UserLoginRequest {
    user_id: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct UserLoginResponse {
    id: Uuid,
    name: String,
    user_id: String,
}

impl From<User> for UserLoginResponse {
    fn from(user: User) -> Self {
        UserLoginResponse {
            id: user.id,
            name: user.name,
            user_id: user.user_id,
        }
    }
}

#[debug_handler]
async fn login(
    State(state): State<AppState>,
    Json(request): Json<UserLoginRequest>,
) -> Result<Json<UserLoginResponse>, StatusCode> {
    match User::authenticate(&state.pool, &request).await {
        Ok(Some(user)) => {
            info!("Login successful: user_id={}", user.user_id);
            Ok(Json(user.into()))
        }
        Ok(None) => {
            info!("Login failed: user_id={}", request.user_id);
            Err(StatusCode::UNAUTHORIZED)
        }
        Err(e) => {
            error!("Login failed: user_id={}, error={}", request.user_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug, Deserialize)]
struct UserGroupCreateRequest {
    name: String,
    user_id_1: Uuid,
    user_id_2: Uuid,
}

#[derive(Debug, Serialize)]
struct UserGroupCreateResponse {
    id: Uuid,
}

fn init_save_dir(path: &str) -> Result<(), std::io::Error> {
    if !Path::new(path).exists() {
        info!("Creating directory: {}", path);
        fs::create_dir_all(path)?;
    }
    info!("Directory exists: {}", path);
    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    init_save_dir("uploads").unwrap();

    let pool = PgPool::connect("postgres://postgres:postgres@localhost/postgres")
        .await
        .unwrap();

    let app_state = AppState {
        pool: Arc::new(pool),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let router = Router::new()
        // health check
        .route("/boku2zenu_king_of_kyodo", post(|| async { "OK" }))
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(app_state)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 1234));
    info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
