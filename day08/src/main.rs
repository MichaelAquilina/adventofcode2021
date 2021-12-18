mod digits;

use digits::Entry;
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    std::io::stdin().read_to_string(&mut buffer)?;

    let entries: Vec<Entry> = buffer
        .lines()
        .map(|l| l.parse())
        .collect::<Result<_, _>>()?;

    println!("Part 1: {:?}", calculate_part_1(&entries));
    println!("Part 2: {:?}", calculate_part_2(&entries)?);

    Ok(())
}

fn calculate_part_1(entries: &[Entry]) -> u32 {
    let mut count = 0;
    let unique_segments = [2, 4, 3, 7];
    for entry in entries {
        for output in &entry.digit_outputs {
            if unique_segments.contains(&output.values.len()) {
                count += 1;
            }
        }
    }
    count
}

fn calculate_part_2(entries: &[Entry]) -> Result<u32, Box<dyn Error>> {
    let mut sum = 0;

    for entry in entries {
        sum += entry.decode()?;
    }

    Ok(sum)
}
