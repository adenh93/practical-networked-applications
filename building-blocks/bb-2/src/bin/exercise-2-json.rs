use anyhow::Result;
use bb_2::Move;

fn main() -> Result<()> {
    let move_down = Move::Down(42);
    let mut buffer = vec![];

    serde_json::to_writer(&mut buffer, &move_down)?;
    let result: Move = serde_json::from_slice(&buffer)?;
    assert_eq!(result, move_down);

    Ok(())
}
