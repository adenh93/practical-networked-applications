use fake::Dummy;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Dummy)]
pub enum Move {
    Up(u8),
    Right(u8),
    Down(u8),
    Left(u8),
}
