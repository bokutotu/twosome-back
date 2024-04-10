use axum::{extract::State, http::StatusCode, Json};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::error;
use uuid::Uuid;

use crate::{
    db::{
        group::{GroupCreate, GroupId},
        user::{UserDB, UserId},
    },
    AppState, UserGroup,
};

#[derive(Debug, Deserialize)]
pub struct UserGroupCreateRequest {
    name: String,
    user_id: Uuid,
}

#[debug_handler]
pub async fn create_group(
    State(state): State<AppState>,
    Json(request): Json<UserGroupCreateRequest>,
) -> Result<Json<Uuid>, StatusCode> {
    let group = GroupCreate::new(request.name.clone());
    let group_id = match group.create(&state.pool).await {
        Ok(id) => id,
        Err(e) => {
            error!("Failed to create group: name={}, error={}", request.name, e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let user_group = UserGroup::new(request.user_id, group_id.uuid());
    match user_group.insert(&state.pool).await {
        Ok(_) => (),
        Err(e) => {
            error!(
                "Failed to create user_group: user_id={}, group_id={}, error={}",
                request.user_id,
                group_id.uuid(),
                e
            );
            group_id.remove(&state.pool).await.unwrap();
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(group_id.uuid()))
}
#[derive(Debug, Deserialize)]
pub struct UserGroupGetRequest {
    user_id: Uuid,
}
#[derive(Debug, Serialize)]
pub struct GetUserGroupResponse {
    group_id: Uuid,
    name: String,
    users: Vec<UserInfo>,
}
#[derive(Debug, Serialize)]
struct UserInfo {
    name: String,
    user_id: Uuid,
}
impl From<UserDB> for UserInfo {
    fn from(user: UserDB) -> Self {
        UserInfo {
            name: user.name().to_string(),
            user_id: user.id(),
        }
    }
}

async fn convert_group_id_to_response(
    group_id: &GroupId,
    pool: &PgPool,
) -> Result<GetUserGroupResponse, sqlx::Error> {
    let group = group_id.get(pool).await?;
    let user_ids = group_id.get_belong_user_ids(pool).await?;
    let mut users = Vec::new();
    for user_id in user_ids {
        let user = user_id.get_user_db(pool).await?;
        users.push(user);
    }
    Ok(GetUserGroupResponse {
        group_id: group_id.uuid(),
        name: group.name().to_string(),
        users: users.into_iter().map(UserInfo::from).collect(),
    })
}

#[debug_handler]
pub async fn get_groups(
    State(state): State<AppState>,
    Json(request): Json<UserGroupGetRequest>,
) -> Result<Json<Vec<GetUserGroupResponse>>, StatusCode> {
    let group_ids = UserId(request.user_id.clone())
        .get_group_ids(&state.pool)
        .await
        .map_err(|e| {
            error!(
                "Failed to get group_ids: user_id={}, error={}",
                request.user_id, e
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    let mut groups = Vec::new();
    for group_id in group_ids {
        match convert_group_id_to_response(&group_id, &state.pool).await {
            Ok(group) => groups.push(group),
            Err(e) => {
                error!(
                    "Failed to convert group_id to response: group_id={}, error={}",
                    group_id.uuid(),
                    e
                );
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }
    Ok(Json(groups))
}

#[derive(Debug, Deserialize)]
pub struct AddGroupUserRequest {
    user_id: Uuid,
    group_id: Uuid,
}

#[debug_handler]
pub async fn add_group_user(
    State(state): State<AppState>,
    Json(request): Json<AddGroupUserRequest>,
) -> StatusCode {
    let user_group = UserGroup::new(request.user_id, request.group_id);
    match user_group.insert(&state.pool).await {
        Ok(_) => StatusCode::OK,
        Err(e) => {
            error!(
                "Failed to create user_group: user_id={}, group_id={}, error={}",
                request.user_id, request.group_id, e
            );
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
