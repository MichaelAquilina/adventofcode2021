mod vector;

use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use vector::{Point, Vector, VectorType};

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    std::io::stdin().read_to_string(&mut buffer)?;

    let mut vectors = vec![];
    for line in buffer.lines() {
        vectors.push(line.parse::<Vector>()?);
    }

    let part_1 = calculate_part_1(&vectors);
    println!("Part 1: {}", part_1);

    let part_2 = calculate_part_2(&vectors);
    println!("Part 2: {}", part_2);

    Ok(())
}

fn calculate_part_1(vectors: &[Vector]) -> usize {
    // only straight lined vectors
    let vectors: Vec<&Vector> = vectors
        .iter()
        .filter(|v| v.get_type() != VectorType::Diagonal)
        .collect();

    calculate(&vectors)
}

fn calculate_part_2(vectors: &[Vector]) -> usize {
    let vectors: Vec<&Vector> = vectors.iter().collect();
    calculate(&vectors)
}

fn calculate(vectors: &[&Vector]) -> usize {
    let mut map: HashMap<Point, u32> = HashMap::new();
    for vector in vectors {
        for point in vector.get_path() {
            let entry = map.entry(point).or_insert(0);
            *entry += 1;
        }
    }

    let count = map.values().filter(|x| *x > &1).count();
    count
}
