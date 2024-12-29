use std::fs;

pub fn read_input(file_path: &str) -> Vec<Vec<i32>> {
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