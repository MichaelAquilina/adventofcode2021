use thiserror::Error;

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash, PartialOrd)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug)]
pub struct Vector {
    pub point_1: Point,
    pub point_2: Point,
}

#[derive(Debug, PartialEq)]
pub enum VectorType {
    Horizontal,
    Vertical,
    Diagonal,
}

impl Vector {
    pub fn get_path(&self) -> Vec<Point> {
        let mut points = vec![];
        let x = if self.point_1.x <= self.point_2.x {
            1
        } else {
            -1
        };
        let y = if self.point_1.y <= self.point_2.y {
            1
        } else {
            -1
        };
        let increase = match self.get_type() {
            VectorType::Vertical => Point { x: 0, y },
            VectorType::Horizontal => Point { x, y: 0 },
            // guaranteed to be 45 degree angles
            VectorType::Diagonal => Point { x, y },
        };

        let mut current = self.point_1;
        points.push(self.point_1);

        while current != self.point_2 {
            current = current + increase;
            points.push(current);
        }

        points
    }

    pub fn get_type(&self) -> VectorType {
        if self.point_1.x == self.point_2.x {
            VectorType::Vertical
        } else if self.point_1.y == self.point_2.y {
            VectorType::Horizontal
        } else {
            VectorType::Diagonal
        }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Error)]
pub enum VectorParseError {
    #[error("Missing {0}")]
    MissingValue(String),
    #[error("Invalid integer coordinate")]
    InvalidInt(#[from] std::num::ParseIntError),
}

impl std::str::FromStr for Vector {
    type Err = VectorParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut tokens = value.split(" -> ");
        let point_1 = tokens
            .next()
            .ok_or_else(|| VectorParseError::MissingValue("point_1".to_string()))?
            .parse()?;
        let point_2 = tokens
            .next()
            .ok_or_else(|| VectorParseError::MissingValue("point_2".to_string()))?
            .parse()?;

        Ok(Vector { point_1, point_2 })
    }
}

impl std::str::FromStr for Point {
    type Err = VectorParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let mut tokens = value.split(',');
        let x = tokens
            .next()
            .ok_or_else(|| VectorParseError::MissingValue("x".to_string()))?
            .parse()?;
        let y = tokens
            .next()
            .ok_or_else(|| VectorParseError::MissingValue("y".to_string()))?
            .parse()?;

        Ok(Point { x, y })
    }
}
