use bcrypt::{hash, verify, DEFAULT_COST};
use sqlx::PgPool;
use tracing::info;
use uuid::Uuid;

use super::group::GroupId;

#[derive(Debug, Clone)]
pub struct UserDB {
    id: Uuid,
    name: String,
    user_id: String,
}
impl UserDB {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn user_id(&self) -> &str {
        &self.user_id
    }

    pub fn new(name: String, user_id: String) -> Self {
        UserDB {
            id: Uuid::new_v4(),
            name,
            user_id,
        }
    }

    pub async fn register(&self, pool: &PgPool, password: String) -> Result<(), sqlx::Error> {
        let hashed_password =
            hash(&password, DEFAULT_COST).map_err(|_| sqlx::Error::PoolTimedOut)?;

        sqlx::query!(
            r#"
            INSERT INTO users (id, name, password, user_id)
            VALUES ($1, $2, $3, $4)
            "#,
            self.id,
            self.name,
            hashed_password,
            self.user_id
        )
        .execute(pool)
        .await?;

        info!(
            "New user registered: name={}, id={}, user_id={}",
            self.name, self.id, self.user_id
        );
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct UserAuth {
    user_id: String,
    password: String,
}
impl UserAuth {
    pub fn new(user_id: String, password: String) -> Self {
        UserAuth { user_id, password }
    }
    pub fn user_id(&self) -> &str {
        &self.user_id
    }
    pub fn password(&self) -> &str {
        &self.password
    }
    pub async fn authenticate(&self, pool: &PgPool) -> Result<Option<UserDB>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT id, user_id, name, password FROM users WHERE user_id = $1
            "#,
            self.user_id
        )
        .fetch_optional(pool)
        .await?;

        match result {
            Some(row) => {
                let stored_password = row.password;
                if verify(&self.password, &stored_password).unwrap_or(false) {
                    info!(
                        "User authenticated: user_id={} id={} name={}",
                        row.user_id, row.id, row.name
                    );
                    Ok(Some(UserDB {
                        id: row.id,
                        name: row.name,
                        user_id: row.user_id,
                    }))
                } else {
                    info!("Authentication failed for user: user_id={}", self.user_id);
                    Ok(None)
                }
            }
            None => {
                info!("User not found: user_id={}", self.user_id);
                Ok(None)
            }
        }
    }
}

pub struct UserId(pub Uuid);
impl UserId {
    pub async fn get_user_db(&self, pool: &PgPool) -> Result<UserDB, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT id, name, user_id FROM users WHERE id = $1
            "#,
            self.0
        )
        .fetch_one(pool)
        .await?;

        Ok(UserDB {
            id: result.id,
            name: result.name,
            user_id: result.user_id,
        })
    }
    pub async fn get_group_ids(&self, pool: &PgPool) -> Result<Vec<GroupId>, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            SELECT group_id FROM user_groups WHERE user_id = $1
            "#,
            self.0
        )
        .fetch_all(pool)
        .await?;

        Ok(result
            .iter()
            .map(|row| GroupId::new(row.group_id))
            .collect())
    }
}
