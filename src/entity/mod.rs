pub mod group;
pub mod group_with_user_info;
pub mod user;
pub mod user_group;

use std::marker::PhantomData;

use uuid::Uuid;

#[derive(Clone, Debug, PartialEq)]
pub struct Id<T> {
    id: Uuid,
    _maker: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new(id: Uuid) -> Self {
        Id {
            id,
            _maker: PhantomData,
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }
}
