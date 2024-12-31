use std::fs;

pub fn read_input(file_path: &str) -> String {
    let input = fs::read_to_string(file_path)
        .expect("Failed to read input file");

    input
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