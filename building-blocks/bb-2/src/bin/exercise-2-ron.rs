use anyhow::Result;
use bb_2::Move;

fn main() -> Result<()> {
    let move_left = Move::Left(15);
    let mut buffer = vec![];

    ron::ser::to_writer(&mut buffer, &move_left)?;
    let result: Move = ron::de::from_bytes(&buffer)?;
    assert_eq!(result, move_left);

    Ok(())
}
