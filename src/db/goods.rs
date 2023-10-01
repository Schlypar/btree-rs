use crate::db::fixed_str::Fixed;
use serde::{Deserialize, Serialize};

pub use super::person::Person;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Crate {
    pub sender: Person,
    pub receiver: Person,
    pub goods_name: Fixed,
    pub producer: Fixed,
    pub goods_id: u64,
}

impl Crate {
    pub fn new(
        sender: Person,
        receiver: Person,
        goods_name: Fixed,
        producer: Fixed,
        goods_id: u64,
    ) -> Self {
        Crate {
            goods_name,
            producer,
            sender,
            receiver,
            goods_id,
        }
    }
}
