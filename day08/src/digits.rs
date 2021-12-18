use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Entry {
    pub signal_patterns: Vec<Pattern>,
    pub digit_outputs: Vec<Pattern>,
}

#[derive(PartialEq, Clone)]
pub struct Pattern {
    pub values: Vec<char>,
}

impl fmt::Debug for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.values.iter().collect::<String>())
    }
}

impl std::str::FromStr for Pattern {
    type Err = Box<dyn Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut values: Vec<char> = value.chars().collect();
        values.sort_unstable();
        Ok(Self { values })
    }
}

impl Pattern {
    fn add(&self, other: &Pattern) -> Self {
        let mut values: HashSet<char> = HashSet::new();
        values.extend(&self.values);
        values.extend(&other.values);
        let mut values: Vec<char> = values.drain().collect();
        values.sort_unstable();

        Self { values }
    }

    fn deduct(&self, other: &Pattern) -> Self {
        let values: Vec<char> = self
            .values
            .iter()
            .filter(|v| !other.values.contains(v))
            .copied()
            .collect();

        Self { values }
    }
}

impl std::str::FromStr for Entry {
    type Err = Box<dyn Error>;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut tokens = value.split(" | ");
        let signal_patterns = tokens.next().ok_or("missing signal patterns")?;
        let digit_outputs = tokens.next().ok_or("missing digit output")?;

        let signal_patterns = signal_patterns
            .split(' ')
            .map(|x| x.parse())
            .collect::<Result<Vec<Pattern>, _>>()?;
        let digit_outputs = digit_outputs
            .split(' ')
            .map(|x| x.parse())
            .collect::<Result<Vec<Pattern>, _>>()?;

        Ok(Entry {
            signal_patterns,
            digit_outputs,
        })
    }
}

impl Entry {
    pub fn decode(&self) -> Result<u32, Box<dyn Error>> {
        let patterns = &self.signal_patterns;
        let one = get_first(find_by_length(patterns, 2))?;
        let four = get_first(find_by_length(patterns, 4))?;
        let seven = get_first(find_by_length(patterns, 3))?;
        let eight = get_first(find_by_length(patterns, 7))?;

        let a = seven.deduct(&one);

        // 5, 2 and 3
        let mut common_for_d = find_by_length(patterns, 5);
        common_for_d.push(four.clone());

        let d = common(&common_for_d);
        let b = four.deduct(&one).deduct(&d);
        let zero = eight.deduct(&d);

        let mut common_for_g = find_by_length(patterns, 5);
        common_for_g.push(zero.clone());

        let g = common(&common_for_g).deduct(&a);
        let nine = four.add(&a).add(&g);
        let e = eight.deduct(&nine);

        let three = eight.deduct(&b).deduct(&e);
        let five_two_three = find_by_length(patterns, 5);
        let five = get_first(get_by(&five_two_three, &b)?)?;
        let two = get_first(get_by(&five_two_three, &e)?)?;
        let six = five.add(&e);

        let mut result = vec![];
        for digit in &self.digit_outputs {
            if digit == &zero {
                result.push('0');
            } else if digit == &one {
                result.push('1');
            } else if digit == &two {
                result.push('2');
            } else if digit == &three {
                result.push('3');
            } else if digit == &four {
                result.push('4');
            } else if digit == &five {
                result.push('5');
            } else if digit == &six {
                result.push('6');
            } else if digit == &seven {
                result.push('7');
            } else if digit == &eight {
                result.push('8');
            } else if digit == &nine {
                result.push('9');
            } else {
                panic!("this isnt right!");
            }
        }
        let result: String = result.into_iter().collect();

        Ok(result.parse()?)
    }
}

fn get_by(patterns: &[Pattern], pattern: &Pattern) -> Result<Vec<Pattern>, Box<dyn Error>> {
    let segment = pattern.values.get(0).ok_or("empty pattern!")?;
    Ok(patterns
        .iter()
        .filter(|p| p.values.contains(segment))
        .cloned()
        .collect())
}

fn get_first(patterns: Vec<Pattern>) -> Result<Pattern, Box<dyn Error>> {
    Ok(patterns.into_iter().next().ok_or("empty!")?)
}

fn find_by_length(patterns: &[Pattern], length: usize) -> Vec<Pattern> {
    patterns
        .iter()
        .filter(|p| p.values.len() == length)
        .cloned()
        .collect()
}

fn common(patterns: &[Pattern]) -> Pattern {
    let mut counter: HashMap<char, usize> = HashMap::new();
    for pattern in patterns {
        for value in &pattern.values {
            let count = counter.entry(*value).or_insert(0);
            *count += 1;
        }
    }

    let mut values: Vec<char> = counter
        .into_iter()
        .filter(|(_, v)| v == &patterns.len())
        .map(|(k, _)| k)
        .collect();
    values.sort_unstable();

    Pattern { values }
}
