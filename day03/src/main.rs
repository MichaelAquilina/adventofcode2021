use std::error::Error;
use std::io::Read;

#[derive(Debug, PartialEq)]
struct PowerConsumption {
    gamma: u32,
    epsilon: u32,
}

#[derive(Debug, PartialEq)]
struct LifeSupportRating {
    oxygen_generator_rating: u32,
    co2_scrubber_rating: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut buffer = String::new();

    std::io::stdin().read_to_string(&mut buffer)?;

    let values: Vec<&str> = buffer.lines().collect();

    let power_consumption = calculate_power_consumption(&values, 12)?;
    println!(
        "Part 1: {} ({:?})",
        power_consumption.epsilon * power_consumption.gamma,
        power_consumption
    );

    let life_support_rating = calculate_life_support_rating(&values, 12)?;
    println!(
        "Part 2: {} ({:?})",
        life_support_rating.oxygen_generator_rating * life_support_rating.co2_scrubber_rating,
        life_support_rating,
    );

    Ok(())
}

fn calculate_frequency(values: &[&str], n: usize) -> Result<u32, Box<dyn Error>> {
    let mut counter = 0;

    for value in values {
        let char = value.chars().nth(n).ok_or("Missing nth digit")?;
        let bit = char.to_digit(2).ok_or("Invalid bit")?;
        counter += bit;
    }

    Ok(counter)
}

fn calculate_power_consumption(
    values: &[&str],
    length: usize,
) -> Result<PowerConsumption, Box<dyn Error>> {
    let mut gamma: Vec<char> = vec![];
    let mut epsilon: Vec<char> = vec![];

    let half = (values.len() / 2) as u32;
    for index in 0..length {
        let counter = calculate_frequency(values, index)?;

        if counter > half {
            gamma.push('1');
            epsilon.push('0');
        } else {
            gamma.push('0');
            epsilon.push('1');
        }
    }

    let gamma = u32::from_str_radix(&gamma.iter().collect::<String>(), 2)?;
    let epsilon = u32::from_str_radix(&epsilon.iter().collect::<String>(), 2)?;

    Ok(PowerConsumption { gamma, epsilon })
}

fn calculate_life_support_rating(
    values: &[&str],
    length: usize,
) -> Result<LifeSupportRating, Box<dyn Error>> {
    let mut oxygen_generator_rating = Vec::from(values);
    let mut co2_scrubber_rating = Vec::from(values);

    for index in 0..length {
        let oxygen_counter = calculate_frequency(&oxygen_generator_rating, index)?;
        let oxygen_half = (oxygen_generator_rating.len() as f32) / 2.0;

        let co2_counter = calculate_frequency(&co2_scrubber_rating, index)?;
        let co2_half = (co2_scrubber_rating.len() as f32) / 2.0;

        let most_common_bit = if oxygen_counter >= oxygen_half.ceil() as u32 {
            '1'
        } else {
            '0'
        };

        let least_common_bit = if co2_counter < co2_half.ceil() as u32 {
            '1'
        } else {
            '0'
        };

        if oxygen_generator_rating.len() > 1 {
            oxygen_generator_rating = oxygen_generator_rating
                .into_iter()
                .filter(|r| r.chars().nth(index) == Some(most_common_bit))
                .collect();
        }

        if co2_scrubber_rating.len() > 1 {
            co2_scrubber_rating = co2_scrubber_rating
                .into_iter()
                .filter(|r| r.chars().nth(index) == Some(least_common_bit))
                .collect();
        }
    }
    let oxygen_generator_rating = oxygen_generator_rating
        .pop()
        .ok_or("Unexpected empty values")?;
    let oxygen_generator_rating = u32::from_str_radix(oxygen_generator_rating, 2)?;

    let co2_scrubber_rating = co2_scrubber_rating.pop().ok_or("Unexpected empty values")?;
    let co2_scrubber_rating = u32::from_str_radix(co2_scrubber_rating, 2)?;

    Ok(LifeSupportRating {
        oxygen_generator_rating,
        co2_scrubber_rating,
    })
}

#[cfg(test)]
mod test_examples {
    use super::*;

    #[test]
    fn test_example() -> Result<(), Box<dyn Error>> {
        let values = vec![
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];

        let result = calculate_power_consumption(&values, 5)?;
        assert_eq!(
            result,
            PowerConsumption {
                gamma: 22,
                epsilon: 9,
            }
        );

        let result = calculate_life_support_rating(&values, 5)?;
        assert_eq!(
            result,
            LifeSupportRating {
                oxygen_generator_rating: 23,
                co2_scrubber_rating: 10,
            }
        );

        Ok(())
    }
}
