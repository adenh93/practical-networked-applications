use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs::OpenOptions, io::Seek};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Move {
    Up(u32),
    Right(u32),
    Down(u32),
    Left(u32),
}

fn main() -> Result<()> {
    let move_up = Move::Up(7);

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open("./output/exercise-1.json")?;

    serde_json::to_writer(&file, &move_up)?;
    file.seek(std::io::SeekFrom::Start(0))?;
    let result = serde_json::from_reader(&file)?;
    assert_eq!(move_up, result);

    Ok(())
}
