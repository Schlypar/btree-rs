use crate::db::fixed_str::Fixed;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Person {
    pub name: Fixed,
    pub surname: Fixed,
    pub patronymic: Fixed,
    pub post_index: u32,
}

impl Person {
    pub fn new(name: Fixed, surname: Fixed, patronymic: Fixed, post_index: u32) -> Self {
        Person {
            name,
            surname,
            patronymic,
            post_index,
        }
    }
}
