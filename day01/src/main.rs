use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    std::io::stdin().read_to_string(&mut buffer)?;

    let mut values: Vec<u32> = vec![];
    for line in buffer.lines() {
        let value = line.parse()?;
        values.push(value);
    }

    let part_1 = get_number_of_increases(&values, 1);
    println!("Part 1: {}", part_1);

    let part_2 = get_number_of_increases(&values, 3);
    println!("Part 2: {}", part_2);

    Ok(())
}

fn get_number_of_increases(values: &[u32], window_size: usize) -> u32 {
    let mut count = 0;
    let mut previous: Option<u32> = None;
    for window in values.windows(window_size) {
        let value: u32 = window.iter().sum();
        if let Some(prev) = previous {
            if value > prev {
                count += 1;
            }
        }
        previous = Some(value);
    }
    return count;
}
