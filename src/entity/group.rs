use crate::{db::group::get_group_by_group_id_db, AppState};

use super::Id;

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    id: Id<Group>,
    name: String,
}

impl Group {
    pub fn new(id: Id<Group>, name: String) -> Self {
        Group { id, name }
    }

    pub fn get_id(&self) -> Id<Group> {
        self.id.clone()
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

pub async fn get_group_by_group_id_entity(
    app: &AppState,
    group_id: Id<Group>,
) -> Result<Group, sqlx::Error> {
    let result = get_group_by_group_id_db(&app.pool, group_id).await?;

    Ok(Group::new(
        Id::<Group>::new(result.id()),
        result.name().to_string(),
    ))
}
