use anyhow::Result;
use bb_2::Move;
use fake::{Fake, Faker};
use std::{fs::OpenOptions, io::Seek};

const NUM_OF_MOVES: usize = 1_000;

fn main() -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .read(true)
        .open("./output/exercise-3.json")?;

    for _ in 0..NUM_OF_MOVES {
        let rand_move: Move = Faker.fake();
        let document = bson::to_document(&rand_move)?;
        document.to_writer(&file)?;
    }

    file.seek(std::io::SeekFrom::Start(0))?;

    while let Ok(de_move) = bson::from_reader::<_, Move>(&mut file) {
        println!("Deserialized move: {de_move:?}");
    }

    Ok(())
}
