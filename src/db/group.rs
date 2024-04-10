use sqlx::PgPool;
use uuid::Uuid;

use super::user::UserId;

pub struct GroupCreate {
    name: String,
}
impl GroupCreate {
    pub fn new(name: String) -> Self {
        GroupCreate { name }
    }
    pub async fn create(&self, pool: &PgPool) -> Result<GroupId, sqlx::Error> {
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

        Ok(GroupId(id))
    }
}
#[derive(Debug, Clone)]
pub struct GroupId(Uuid);
impl GroupId {
    pub fn new(id: Uuid) -> Self {
        GroupId(id)
    }
    pub fn uuid(&self) -> Uuid {
        self.0
    }
    pub async fn remove(&self, pool: &PgPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM groups WHERE id = $1
            "#,
            self.0
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get_belong_user_ids(&self, pool: &PgPool) -> Result<Vec<UserId>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT user_id FROM user_groups WHERE group_id = $1
            "#,
            self.0
        )
        .fetch_all(pool)
        .await?;

        Ok(result.iter().map(|row| UserId(row.user_id)).collect())
    }

    pub async fn get(&self, pool: &PgPool) -> Result<GroupDB, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT id, name FROM groups WHERE id = $1
            "#,
            self.0
        )
        .fetch_one(pool)
        .await?;

        Ok(GroupDB {
            id: GroupId(result.id),
            name: result.name,
        })
    }
}
#[derive(Debug)]
pub struct GroupDB {
    id: GroupId,
    name: String,
}
impl GroupDB {
    pub fn id(&self) -> Uuid {
        self.id.uuid()
    }
    pub fn name(&self) -> &str {
        &self.name
    }
}
