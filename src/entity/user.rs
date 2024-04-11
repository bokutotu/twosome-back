use uuid::Uuid;

use crate::{
    db::user::{get_user_by_id, UserDB},
    AppState,
};

use super::Id;

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    id: Id<User>,
    name: String,
    user_id: String,
}

impl From<UserDB> for User {
    fn from(user: UserDB) -> Self {
        User {
            id: Id::new(user.id()),
            name: user.name().to_string(),
            user_id: user.user_id().to_string(),
        }
    }
}

impl User {
    pub fn new(id: Id<User>, name: String, user_id: String) -> Self {
        User { id, name, user_id }
    }

    pub fn get_id(&self) -> Uuid {
        self.id.get_id()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }
}

pub async fn get_user_by_user_id_entity(
    state: &AppState,
    user_id: Id<User>,
) -> Result<User, sqlx::Error> {
    Ok(User::from(get_user_by_id(&state.pool, user_id).await?))
}
