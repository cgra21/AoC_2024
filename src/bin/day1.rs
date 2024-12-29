use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_path = &args[1];

    let (mut left_list, mut right_list) = read_input(file_path);

    let total_distance = calculate_total_distance(&mut left_list, &mut right_list);

    println!("Total Distance:\n {total_distance}")

}

fn read_input(file_path: &str) -> (Vec<i32>, Vec<i32>) {
    let input = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in input.lines() {
        let nums: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().expect("Failed to parse number"))
            .collect();
        left_list.push(nums[0]);
        right_list.push(nums[1]);
    }

    (left_list, right_list)
}

fn calculate_total_distance(left_list: &mut Vec<i32>, right_list: &mut Vec<i32>) -> i32 {
    left_list.sort();
    right_list.sort();

    left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l,r)| (l - r).abs())
        .sum()
}