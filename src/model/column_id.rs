use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize
)]
pub struct ColumnId(Uuid);

impl ColumnId {
    pub fn new() -> Self {
        Self(
            Uuid::new_v4()
        )
    }
    pub fn parse_column_id(
        value: &str
    ) -> Self {
        Self(
            Uuid::parse_str(value)
            .expect("Invalid ColumnId")
        )
    }
}