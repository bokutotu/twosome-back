pub mod controllers;
pub mod db;
pub mod entity;

use std::{fs, net::SocketAddr, path::Path, sync::Arc};

use axum::{routing::post, Router};
use sqlx::PgPool;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

use crate::controllers::group::{create_group, get_groups};
use crate::controllers::user::{login, register};

#[derive(Clone)]
pub struct AppState {
    pool: Arc<PgPool>,
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
        .route("/create_group", post(create_group))
        .route("/get_groups", post(get_groups))
        .with_state(app_state)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 1234));
    info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
