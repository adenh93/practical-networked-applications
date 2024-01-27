use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Move {
    Up(u32),
    Right(u32),
    Down(u32),
    Left(u32),
}
