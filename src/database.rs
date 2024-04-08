use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    id: Uuid,
    name: String,
    password: String,
}

impl User {
    pub fn new(name: &str, password: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.to_string(),
            password: password.to_string(),
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct Group {
    id: Uuid,
    name: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct UserGroup {
    user_id: Uuid,
    group_id: Uuid,
}

#[derive(Debug, sqlx::FromRow)]
pub struct PostUrl {
    id: Uuid,
    user_id: Uuid,
    group_id: Uuid,
    content: String,
    status: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct PostImage {
    id: Uuid,
    user_id: Uuid,
    group_id: Uuid,
    content: String,
    created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct PostVideo {
    id: Uuid,
    user_id: Uuid,
    group_id: Uuid,
    content: String,
    created_at: DateTime<Utc>,
}
