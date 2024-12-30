use std::fs;

pub fn read_input(file_path: &str) -> String {
    let input = fs::read_to_string(file_path)
        .expect("Failed to read input file");

    input
}