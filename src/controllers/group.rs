use axum::{extract::State, http::StatusCode, Json};
use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use tracing::error;
use uuid::Uuid;

use crate::{
    db::group::{remove_group_by_group_id, GroupCreate},
    entity::{
        group_with_user_info::{get_group_with_user_info_by_group_id, GroupWithUserInfo},
        user::User,
        user_group::get_user_group_by_user_id_entity,
        Id,
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

    let user_group = UserGroup::new(request.user_id, group_id.get_id());
    match user_group.insert(&state.pool).await {
        Ok(_) => (),
        Err(e) => {
            error!(
                "Failed to create user_group: user_id={}, group_id={}, error={}",
                request.user_id,
                group_id.get_id(),
                e
            );
            remove_group_by_group_id(&state.pool, group_id)
                .await
                .unwrap();
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    Ok(Json(group_id.get_id()))
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
impl From<&GroupWithUserInfo> for GetUserGroupResponse {
    fn from(value: &GroupWithUserInfo) -> Self {
        let group_id = value.get_group().get_id().get_id();
        let name = value.get_group().get_name().to_string();
        let belogn_users = value.get_belongs_users();
        let users = belogn_users.into_iter().map(UserInfo::from).collect();
        GetUserGroupResponse {
            group_id,
            name,
            users,
        }
    }
}
#[derive(Debug, Serialize)]
struct UserInfo {
    name: String,
    user_id: String,
    id: Uuid,
}
impl From<&User> for UserInfo {
    fn from(user: &User) -> Self {
        UserInfo {
            name: user.get_name().to_string(),
            user_id: user.get_user_id().to_string(),
            id: user.get_id(),
        }
    }
}

#[debug_handler]
pub async fn get_groups(
    State(state): State<AppState>,
    Json(request): Json<UserGroupGetRequest>,
) -> Result<Json<Vec<GetUserGroupResponse>>, StatusCode> {
    let user_groups = get_user_group_by_user_id_entity(&state, Id::<User>::new(request.user_id))
        .await
        .map_err(|e| {
            error!(
                "Failed to get user_group: user_id={}, error={}",
                request.user_id, e
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;
    if user_groups.is_empty() {
        return Ok(Json(vec![]));
    }
    let mut result = Vec::new();
    for user_group in user_groups {
        let group_with_user_info =
            get_group_with_user_info_by_group_id(&state, user_group.group_id())
                .await
                .map_err(|e| {
                    error!(
                        "Failed to get group_with_user_info: group_id={:?}, error={}",
                        user_group.group_id(),
                        e
                    );
                    StatusCode::INTERNAL_SERVER_ERROR
                })?;
        result.push(GetUserGroupResponse::from(&group_with_user_info))
    }
    Ok(Json(result))
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
