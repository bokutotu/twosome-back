use sqlx::PgPool;
use uuid::Uuid;

use crate::entity::{group::Group, user::User, user_group::UserGroup, Id};

pub struct UserGroupDB {
    user_id: Uuid,
    group_id: Uuid,
}

impl UserGroupDB {
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
    pub fn group_id(&self) -> Uuid {
        self.group_id
    }
}

impl From<UserGroup> for UserGroupDB {
    fn from(user_group: UserGroup) -> Self {
        UserGroupDB {
            user_id: user_group.user_id().get_id(),
            group_id: user_group.group_id().get_id(),
        }
    }
}

impl From<UserGroupDB> for UserGroup {
    fn from(user_group_db: UserGroupDB) -> Self {
        UserGroup::new(
            Id::<User>::new(user_group_db.user_id),
            Id::<Group>::new(user_group_db.group_id),
        )
    }
}

pub async fn get_user_group_by_group_id_db(
    pool: &PgPool,
    group_id: Id<Group>,
) -> Result<Vec<UserGroupDB>, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT user_id, group_id FROM user_groups WHERE group_id = $1
        "#,
        group_id.get_id()
    )
    .fetch_all(pool)
    .await?;

    Ok(result
        .iter()
        .map(|row| UserGroupDB {
            user_id: row.user_id,
            group_id: row.group_id,
        })
        .collect())
}

pub async fn get_user_group_by_user_id_db(
    pool: &PgPool,
    user_id: Id<User>,
) -> Result<Vec<UserGroupDB>, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT user_id, group_id FROM user_groups WHERE user_id = $1
        "#,
        user_id.get_id()
    )
    .fetch_all(pool)
    .await?;

    Ok(result
        .iter()
        .map(|row| UserGroupDB {
            user_id: row.user_id,
            group_id: row.group_id,
        })
        .collect())
}
