use crate::AppState;

use super::{
    group::{get_group_by_group_id_entity, Group},
    user::{get_user_by_user_id_entity, User},
    user_group::get_user_group_by_group_id_entity,
    Id,
};

#[derive(Debug, Clone, PartialEq)]
pub struct GroupWithUserInfo {
    group: Group,
    belongs_users: Vec<User>,
}
impl GroupWithUserInfo {
    pub fn get_group(&self) -> &Group {
        &self.group
    }

    pub fn get_belongs_users(&self) -> &[User] {
        &self.belongs_users
    }
}

pub async fn get_group_with_user_info_by_group_id(
    state: &AppState,
    group_id: Id<Group>,
) -> Result<GroupWithUserInfo, sqlx::Error> {
    let group = get_group_by_group_id_entity(&state, group_id.clone()).await?;
    let users_groups = get_user_group_by_group_id_entity(state, group_id).await?;
    let mut belongs_users = Vec::new();
    for user_group in users_groups {
        let user = get_user_by_user_id_entity(state, user_group.user_id()).await?;
        belongs_users.push(user);
    }

    Ok(GroupWithUserInfo {
        group,
        belongs_users,
    })
}
