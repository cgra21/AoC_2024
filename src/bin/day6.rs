use std::hash::Hash;
use std::result;
use std::{collections::HashSet, env};

use advent_of_code::utils::{self, Point};
use advent_of_code::utils::Direction;

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines() // Split the input into lines
        .map(|line| line.trim())
        .map(|line| line.chars().collect()) // Convert each line into a Vec<char>
        .collect() // Collect all Vec<char> into a Vec<Vec<char>>
}

fn guard_patrol(grid: Vec<Vec<char>>) -> HashSet<Point> {

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


    let walls: HashSet<Point> = 
        grid.iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(j, &value)| {
                        if value == '#' {
                            Some(Point::new(j as isize, i as isize))
                        } else {
                            None
                        }
                    })
            })
            .collect();

    let mut direction = Direction::North;
    println!("Starting Direction: {direction:?}");

    println!("Found '^' at position ({}, {})", position.y, position.x);
    visited.insert(position);



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

        if walls.contains(&next_position) {
            direction = direction.turn_right();
        } else {
            position.move_by(dx, dy);
            visited.insert(position);
        }
    }

    visited
    
}

fn part_2(grid: Vec<Vec<char>>, mut visited: HashSet<Point>) -> usize {

    let mut loop_count = 0;

    let rows = grid.len();
    let cols = grid[0].len();

    let mut position: Point = 
        grid.iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &value)| (i, j, value)))
            .find(|&(_, _, value)| value == '^')
            .map(|(row, col, _)| Point::new(col as isize, row as isize))
            .expect("Starting position not found"); // Extract only the position

    let mut walls: HashSet<Point> = 
        grid.iter()
            .enumerate()
            .flat_map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(j, &value)| {
                        if value == '#' {
                            Some(Point::new(j as isize, i as isize))
                        } else {
                            None
                        }
                    })
            })
            .collect();
        
    let original_position = position.clone();

    visited.remove(&original_position);

    let results = visited
            .iter()
            .filter(|new_wall| {

                let mut position = position.clone();
                let mut direction = Direction::North;

                let mut visited_positions: HashSet<(
                    Point,
                    Direction,
                )> = HashSet::from([(position, 
                    direction.clone(),
                    )]);

                loop {
                    let (dx, dy) = direction.to_vector();

                    let next_position = Point::new(
                        position.x + dx,
                        position.y + dy,
                    );

                    if walls.contains(&next_position) 
                        || &&next_position == new_wall 
                        {
                            direction = direction.turn_right();
                            continue;
                        }

                        if visited_positions.contains(&(next_position, direction)) {
                            break true;
                        }
                    


                        if (0..rows as isize)
                            .contains(&next_position.y)
                            && (0..cols as isize)
                                .contains(&next_position.x) {
                                    position = next_position;
                                    visited_positions.insert((
                                        position,
                                        direction,
                                    ));
                                    continue;
                        } else {
                            break false;
                        }

            
                }
            })
            .count();
    results
}


fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_path = &args[1];

    let input = utils::read_input(file_path);

    let grid = parse_input(&input);

    let path = guard_patrol(grid.clone());
    let path_length = path.len();

    println!("Number of spots visited: {path_length}");

    let potential_walls = part_2(grid.clone(), path);

    println!("Potential Walls: {potential_walls}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_guard_path() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

        let grid = parse_input(&input);

        let visited = guard_patrol(grid.clone());

        assert_eq!(visited.len(), 41);

    }


    #[test]
    fn test_part_2() {
        let input = "....#.....
....^....#
..........
..#.......
.......#..
..........
.#........
........#.
#.........
......#...";

        let grid = parse_input(&input);

        let visited = guard_patrol(grid.clone());

        let result = part_2(grid, visited);

        assert_eq!(result, 6);
    }

}