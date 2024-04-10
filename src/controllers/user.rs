use axum::{extract::State, http::StatusCode, Json};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use uuid::Uuid;

use crate::{
    db::user::{UserAuth, UserDB},
    AppState,
};

#[derive(Debug, Deserialize)]
pub struct UserLoginRequest {
    user_id: String,
    password: String,
}

impl From<UserLoginRequest> for UserAuth {
    fn from(request: UserLoginRequest) -> Self {
        UserAuth::new(request.user_id, request.password)
    }
}

#[derive(Debug, Serialize)]
pub struct UserLoginResponse {
    id: Uuid,
    name: String,
    user_id: String,
}
impl From<UserDB> for UserLoginResponse {
    fn from(user: UserDB) -> Self {
        UserLoginResponse {
            id: user.id(),
            name: user.name().to_string(),
            user_id: user.user_id().to_string(),
        }
    }
}
#[debug_handler]
pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<UserLoginRequest>,
) -> Result<Json<UserLoginResponse>, StatusCode> {
    let user_auth = UserAuth::from(request);
    match user_auth.authenticate(&state.pool).await {
        Ok(Some(user)) => {
            info!("Login successful: user_id={}", user.user_id().to_string());
            Ok(Json(UserLoginResponse::from(user)))
        }
        Ok(None) => {
            info!("Login failed: user_id={}", user_auth.user_id().to_string());
            Err(StatusCode::UNAUTHORIZED)
        }
        Err(e) => {
            error!("Login failed: user_id={}, error={}", user_auth.user_id(), e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UserRegisterRequest {
    name: String,
    password: String,
    user_id: String,
}

impl UserRegisterRequest {
    fn into_user(self) -> UserDB {
        UserDB::new(self.name, self.user_id)
    }
}

#[derive(Debug, Serialize)]
pub struct UserRegisterResponse {
    id: Uuid,
}
#[debug_handler]
pub async fn register(
    State(state): State<AppState>,
    Json(request): Json<UserRegisterRequest>,
) -> Result<Json<UserRegisterResponse>, StatusCode> {
    let password = request.password.clone();
    let user = request.into_user();
    match user.register(&state.pool, password).await {
        Ok(()) => {
            info!("Registration successful: id={}", user.id());
            Ok(Json(UserRegisterResponse { id: user.id() }))
        }
        Err(e) => {
            error!(
                "Registration failed: user_id={}, error={}",
                user.user_id(),
                e
            );
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
