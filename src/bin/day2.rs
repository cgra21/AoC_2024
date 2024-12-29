use std::env;
use std::fs;

fn read_input(file_path: &str) -> Vec<Vec<i32>> {
    let input = fs::read_to_string(file_path)
        .expect("Failed to read input file");

    let mut result = Vec::new();

    for line in input.lines() {
        let row: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().expect("Failed to parse line"))
            .collect();
        result.push(row);
        
    }

    result
}

fn check_safe(row: &[i32]) -> bool {

    let mut increasing = true;
    let mut decreasing = true;

    for (prev, next) in row.iter().zip(row.iter().skip(1)) {
        let diff = (prev - next).abs();

        if diff < 1 || diff > 3{
            return false;
        }

        if next < prev {
            increasing = false;
        }

        if next > prev {
            decreasing = false;
        }

    }

    increasing || decreasing
}

fn check_safe_with_dampener(row: &[i32]) -> bool {
    
    if check_safe(row) {
        return true;
    }

    for i in 0..row.len() {
        let mut modified_row = row.to_vec();
        modified_row.remove(i);

        if check_safe(&modified_row) {
            return true;
        }

    }

    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_path = &args[1];

    let results = read_input(file_path);

    let mut count = 0;
    let mut with_dampner = 0;
    let mut row_count = 0;

    for row in results.iter() {
        println!("{row:?}");
        row_count += 1;
        if check_safe(row) {
            count += 1;
        }
    }

    for row in results.iter() {
        if check_safe_with_dampener(row) {
            with_dampner += 1;
        }
    }
    println!("Total reports: {row_count}");
    println!("Total Safe reports {count}");
    println!("Total with dampner {with_dampner}");
}