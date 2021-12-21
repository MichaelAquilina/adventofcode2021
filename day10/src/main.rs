mod bracket;

use std::error::Error;
use std::io::Read;

use bracket::calculate_scores;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    std::io::stdin().read_to_string(&mut buffer)?;

    let scores = calculate_scores(&buffer)?;
    println!("Part 1: {}", scores.corrupted);
    println!("Part 2: {}", scores.incomplete);

    Ok(())
}
