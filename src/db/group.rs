use sqlx::PgPool;
use uuid::Uuid;

use crate::entity::{group::Group, Id};

pub struct GroupCreate {
    name: String,
}
impl GroupCreate {
    pub fn new(name: String) -> Self {
        GroupCreate { name }
    }
    pub async fn create(&self, pool: &PgPool) -> Result<Id<Group>, sqlx::Error> {
        let id = Uuid::new_v4();

        sqlx::query!(
            r#"
            INSERT INTO groups (id, name)
            VALUES ($1, $2)
            "#,
            id,
            self.name
        )
        .execute(pool)
        .await?;

        Ok(Id::<Group>::new(id))
    }
}

#[derive(Debug)]
pub struct GroupDB {
    id: Id<Group>,
    name: String,
}
impl GroupDB {
    pub fn id(&self) -> Uuid {
        self.id.get_id()
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}

pub async fn remove_group_by_group_id(
    pool: &PgPool,
    group_id: Id<Group>,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM groups WHERE id = $1
        "#,
        group_id.get_id()
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_group_by_group_id_db(
    pool: &PgPool,
    group_id: Id<Group>,
) -> Result<GroupDB, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT id, name FROM groups WHERE id = $1
        "#,
        group_id.get_id()
    )
    .fetch_one(pool)
    .await?;

    Ok(GroupDB {
        id: Id::<Group>::new(result.id),
        name: result.name,
    })
}

pub async fn get_group_by_group_id(
    pool: &PgPool,
    group_id: Id<Group>,
) -> Result<GroupDB, sqlx::Error> {
    let result = sqlx::query!(
        r#"
        SELECT id, name FROM groups WHERE id = $1
        "#,
        group_id.get_id()
    )
    .fetch_one(pool)
    .await?;

    Ok(GroupDB {
        id: Id::<Group>::new(result.id),
        name: result.name,
    })
}
