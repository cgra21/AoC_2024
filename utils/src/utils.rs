use std::fs;
use std::ops::{Add, Sub, Mul};

pub fn parse_grid(input: &str) -> Vec<Vec<char>> {
    input
        .lines() // Split the input into lines
        .map(|line| line.trim())
        .map(|line| line.chars().collect()) // Convert each line into a Vec<char>
        .collect() // Collect all Vec<char> into a Vec<Vec<char>>
}

pub fn read_input(file_path: &str) -> String {
    let input = fs::read_to_string(file_path)
        .expect("Failed to read input file");

    input
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x:isize, y:isize) -> Self {
        Point {x, y}
    }

    pub fn move_by(&mut self, dx: isize, dy: isize) {
        self.x = self.x + dx;
        self.y = self.y + dy;
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y
        }
    }
}

impl Mul<isize> for Point {
    type Output = Point;

    fn mul(self, scalar: isize) -> Point {
        Point {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}



pub struct Vectors;

impl Vectors {
    // 
    pub const N: (isize, isize) = (0, 1);
    pub const NE: (isize, isize) = (1, 1);
    pub const E: (isize, isize) = (1, 0);
    pub const SE: (isize, isize) = (1, -1);
    pub const S: (isize, isize) = (0, -1);
    pub const SW: (isize, isize) = (-1, -1);
    pub const W: (isize, isize) = (-1, 0);
    pub const NW: (isize, isize) = (-1, 1);

    pub fn all_directions() -> Vec<(isize, isize)> {
        vec![
            Self::N,
            Self::NE,
            Self::E,
            Self::SE,
            Self::S,
            Self::SW,
            Self::W,
            Self::NW,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    pub fn to_vector(&self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

