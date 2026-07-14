use serde::{Deserialize, Serialize};

#[derive(
    Serialize,
    Deserialize
)]

pub enum Colour {
    Red,
    Blue,
    Yellow,
    Green,
    Purple,
    Amber,
}
