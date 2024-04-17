use crate::{
    db::user_group::{
        create_user_group_db, get_user_group_by_group_id_db, get_user_group_by_user_id_db,
    },
    AppState,
};

use super::{group::Group, user::User, Id};

#[derive(Debug, Clone, PartialEq)]
pub struct UserGroup {
    user_id: Id<User>,
    group_id: Id<Group>,
}

impl UserGroup {
    pub fn new(user_id: Id<User>, group_id: Id<Group>) -> Self {
        UserGroup { user_id, group_id }
    }

    pub fn user_id(&self) -> Id<User> {
        self.user_id.clone()
    }

    pub fn group_id(&self) -> Id<Group> {
        self.group_id.clone()
    }
}

pub async fn get_user_group_by_user_id_entity(
    app: &AppState,
    user_id: Id<User>,
) -> Result<Vec<UserGroup>, sqlx::Error> {
    let result = get_user_group_by_user_id_db(&app.pool, user_id).await?;

    Ok(result.into_iter().map(UserGroup::from).collect())
}

pub async fn get_user_group_by_group_id_entity(
    app: &AppState,
    group_id: Id<Group>,
) -> Result<Vec<UserGroup>, sqlx::Error> {
    let result = get_user_group_by_group_id_db(&app.pool, group_id).await?;

    Ok(result.into_iter().map(UserGroup::from).collect())
}

pub async fn create_user_group(app: &AppState, user_group: UserGroup) -> Result<(), sqlx::Error> {
    create_user_group_db(&app.pool, user_group).await
}
