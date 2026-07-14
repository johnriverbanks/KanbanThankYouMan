use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize
)]

pub struct ColumnId(String);

impl ColumnId {
    pub fn new() -> Self {
        Self(
            Uuid::new_v4()
                .to_string()
        )
    }
    pub fn from_static(value: &str) -> Self {
        Self(
            value.to_string()
        )
    }
}