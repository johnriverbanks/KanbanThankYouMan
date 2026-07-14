use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(
    PartialEq,
    Serialize,
    Deserialize
)]

pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Unknown,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Difficulty::Easy => write!(f, "Easy"),
            Difficulty::Medium => write!(f, "Medium"),
            Difficulty::Hard => write!(f, "Hard"),
            Difficulty::Unknown => write!(f, "Unknown"),
        }
    }
}

