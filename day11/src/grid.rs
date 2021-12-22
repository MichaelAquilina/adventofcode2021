use std::error::Error;

#[derive(Debug)]
pub struct Grid {
    width: usize,
    height: usize,
    values: Vec<u32>,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl std::str::FromStr for Grid {
    type Err = Box<dyn Error>;
    fn from_str(buffer: &str) -> Result<Self, Self::Err> {
        let mut values: Vec<u32> = vec![];
        for line in buffer.lines() {
            for character in line.chars() {
                values.push(character.to_digit(10).ok_or("invalid digit")?);
            }
        }

        // width and height are fixed in this challenge
        Ok(Grid {
            values,
            width: 10,
            height: 10,
        })
    }
}

impl Grid {
    pub fn run_step(&mut self) -> usize {
        // increase energy of all octopuses by 1
        for value in self.values.iter_mut() {
            *value += 1;
        }

        let mut flashed = vec![];
        loop {
            let indexes: Vec<usize> = self
                .values
                .iter()
                .enumerate()
                .filter(|(_, v)| *v > &9)
                .map(|(i, _)| i)
                .filter(|i| !flashed.contains(i))
                .collect();

            if indexes.is_empty() {
                break;
            }

            for index in indexes {
                flashed.push(index);
                self.increase_with_neighbours(index);
            }
        }

        for index in &flashed {
            self.values[*index] = 0;
        }
        flashed.len()
    }

    fn get_coord(&self, index: usize) -> Point {
        let x = index % self.width;
        let y = index / self.height;
        let x = x as i32;
        let y = y as i32;

        Point { x, y }
    }

    fn get_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || y < 0 {
            return None;
        }
        let x = x as usize;
        let y = y as usize;
        if x >= self.width || y >= self.height {
            None
        } else {
            Some((y * self.width) + x)
        }
    }

    fn get_neighbours(&self, point: &Point) -> [Point; 8] {
        [
            Point {
                x: point.x - 1,
                y: point.y - 1,
            },
            Point {
                x: point.x - 1,
                y: point.y,
            },
            Point {
                x: point.x - 1,
                y: point.y + 1,
            },
            Point {
                x: point.x,
                y: point.y - 1,
            },
            Point {
                x: point.x,
                y: point.y + 1,
            },
            Point {
                x: point.x + 1,
                y: point.y - 1,
            },
            Point {
                x: point.x + 1,
                y: point.y,
            },
            Point {
                x: point.x + 1,
                y: point.y + 1,
            },
        ]
    }

    fn increase_with_neighbours(&mut self, index: usize) {
        let point = self.get_coord(index);

        for point in self.get_neighbours(&point) {
            if let Some(index) = self.get_index(point.x, point.y) {
                self.values[index] += 1;
            }
        }
    }
}

#[cfg(test)]
mod test_grid {
    use super::*;

    #[test]
    fn test_example() -> Result<(), Box<dyn Error>> {
        let example = [
            "5483143223",
            "2745854711",
            "5264556173",
            "6141336146",
            "6357385478",
            "4167524645",
            "2176841721",
            "6882881134",
            "4846848554",
            "5283751526",
        ]
        .join("\n");

        let mut grid: Grid = example.parse()?;
        let mut flashes = 0;
        for _ in 0..10 {
            flashes += grid.run_step();
        }
        assert_eq!(flashes, 204);

        for _ in 10..100 {
            flashes += grid.run_step();
        }
        assert_eq!(flashes, 1656);

        // test that they all flashed simultaneously on step 195
        for _ in 100..194 {
            grid.run_step();
        }
        assert_eq!(grid.run_step(), 100);

        Ok(())
    }
}
