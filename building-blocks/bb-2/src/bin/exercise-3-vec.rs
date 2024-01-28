use anyhow::Result;
use bb_2::{Move, ReadBuffer};
use fake::{Fake, Faker};

const NUM_OF_MOVES: usize = 1_000;

fn main() -> Result<()> {
    let mut buffer = ReadBuffer::new();

    for _ in 0..NUM_OF_MOVES {
        let rand_move: Move = Faker.fake();
        let document = bson::to_document(&rand_move)?;
        document.to_writer(&mut buffer)?;
    }

    while let Ok(de_move) = bson::from_reader::<_, Move>(&mut buffer) {
        println!("{de_move:?}");
    }

    Ok(())
}
