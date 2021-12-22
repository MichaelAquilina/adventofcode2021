mod grid;

use grid::Grid;
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    std::io::stdin().read_to_string(&mut buffer)?;

    let mut grid: Grid = buffer.parse()?;

    let mut part1 = vec![];
    let mut part2: Option<usize> = None;
    let mut step = 0;

    while part1.len() < 100 || part2.is_none() {
        step += 1;
        let flashes = grid.run_step();
        if part1.len() < 100 {
            part1.push(flashes);
        }
        if flashes == 100 && part2.is_none() {
            part2 = Some(step);
        }
    }

    println!("Part 1: {}", part1.iter().sum::<usize>());
    println!("Part 2: {:?}", part2);

    Ok(())
}
