use itertools::sorted;
use std::error::Error;
use std::io::Read;

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    std::io::stdin().read_to_string(&mut buffer)?;

    let buffer = buffer.strip_suffix('\n').ok_or("invalid input")?;

    let mut crabs = vec![];
    for value in buffer.split(',') {
        crabs.push(value.parse::<i32>()?);
    }
    println!("Part 1: {}", calculate_part_1(&crabs));
    println!("Part 2: {}", calculate_part_2(&crabs)?);

    Ok(())
}

fn calculate_part_1(crabs: &[i32]) -> i32 {
    let med = median(crabs);
    crabs.iter().map(|x| (med - x).abs()).sum()
}

fn calculate_part_2(crabs: &[i32]) -> Result<u32, Box<dyn Error>> {
    let max = *crabs.iter().max().ok_or("empty")?;
    let min = *crabs.iter().min().ok_or("empty")?;
    let mut result = u32::MAX;

    for value in min..max {
        let cost = calculate_total_real_fuel_cost(crabs, value);
        if cost < result {
            result = cost;
        }
    }

    Ok(result)
}

fn calculate_total_real_fuel_cost(crabs: &[i32], value: i32) -> u32 {
    let mut cost = 0;
    for crab in crabs {
        cost += real_fuel_cost((crab - value).abs() as u32);
    }
    cost
}

fn real_fuel_cost(distance: u32) -> u32 {
    // arithmetic progression
    let n = distance;
    (n * (n + 1)) / 2
}

fn median(values: &[i32]) -> i32 {
    let values = sorted(values).collect::<Vec<&i32>>();
    *values[values.len() / 2]
}
