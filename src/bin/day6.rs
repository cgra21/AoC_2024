use std::{collections::HashSet, env};

use advent_of_code::utils::Point;
use advent_of_code::utils;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines() // Split the input into lines
        .map(|line| line.trim())
        .map(|line| line.chars().collect()) // Convert each line into a Vec<char>
        .collect() // Collect all Vec<char> into a Vec<Vec<char>>
}

fn guard_patrol(grid: Vec<Vec<char>>) -> usize {

    let mut visited: HashSet<Point> = HashSet::new();

    let rows = grid.len();
    let cols = grid[0].len();

    let mut position: Point = 
        grid.iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &value)| (i, j, value)))
            .find(|&(_, _, value)| value == '^')
            .map(|(row, col, _)| Point::new(col as isize, row as isize))
            .expect("Starting position not found"); // Extract only the position

    println!("Found '^' at position ({}, {})", position.y, position.x);
    visited.insert(position);

    let mut direction = utils::Direction::North;
    println!("Starting Direction: {direction:?}");

    loop {
        let (dx, dy) = direction.to_vector();

        let next_position = Point::new(
            position.x + dx,
            position.y + dy,
        );

        if next_position.x as usize >= cols 
            || next_position.y as usize >= rows {
                break;
            }

        let symbol = grid[next_position.y as usize][next_position.x as usize];

        if symbol == '#' {
            direction = direction.turn_right();
        } else {
            position.move_by(dx, dy);
            visited.insert(position);
        }
    }

    visited.len()

}


fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_path = &args[1];

    let input = utils::read_input(file_path);

    let grid = parse_input(&input);

    let wall = grid[0][4];
    println!("{wall}");
    let path_length = guard_patrol(grid);

    println!("Number of spots visited: {path_length}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn main() {

    }
}