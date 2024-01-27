use anyhow::Result;
use bb_2::Move;
use std::{fs::OpenOptions, io::Seek};

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
