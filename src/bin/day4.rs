use advent_of_code::utils;

use std::fs;
use std::env;

fn count_xmas(grid: Vec<Vec<char>>) -> usize {

    let word = "XMAS".chars().collect::<Vec<char>>();
    let directions = utils::Vectors::all_directions();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    let is_word_at = |x: usize, y: usize, dx: isize, dy: isize| -> bool {
        for (i, &ch) in word.iter().enumerate() {
            let nx = x as isize + i as isize * dx;
            let ny = y as isize + i as isize * dy;
            if nx < 0 || ny < 0 || nx >= rows as isize || ny >= cols as isize {
                return false;
            }
            if grid[nx as usize][ny as usize] != ch {
                return false;
            }            
        }
        true
    };

    for x in 0..rows {
        for y in 0..cols {
            for &(dx, dy) in &directions {
                if is_word_at(x, y, dx, dy) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines() // Split the input into lines
        .map(|line| line.trim())
        .map(|line| line.chars().collect()) // Convert each line into a Vec<char>
        .collect() // Collect all Vec<char> into a Vec<Vec<char>>
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_path = &args[1];

    let input = utils::read_input(file_path);

    let grid = parse_input(&input);

    let count = count_xmas(grid);
    println!("The word 'XMAS' appears {} times.", count);
}