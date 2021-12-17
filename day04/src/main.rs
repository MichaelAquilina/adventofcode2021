mod bingo;

use bingo::BingoGame;
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    std::io::stdin().read_to_string(&mut buffer)?;

    let game = BingoGame::parse(&buffer, 5, 5)?;
    let results = game.play()?;

    if let Some(result) = results.first() {
        println!("Part 1: {}", result.get_result()?);
    }
    if let Some(result) = results.last() {
        println!("Part 2: {}", result.get_result()?);
    }

    Ok(())
}
