use std::env;

use utils::utils;

use itertools::Itertools;

const OPERATORS: [char; 2] = ['*', '+'];

fn parse_input(input: &str) -> Vec<(isize, Vec<isize>)> {

    input
        .lines()
        .map(|line| {
            // split into test and values
            let (test, numbers) = line.split_once(':').unwrap();

            // Parse test into isize
            let test = test.trim().parse::<isize>().unwrap();

            let numbers = numbers
                .trim()
                .split_whitespace()
                .map(|n| n.parse::<isize>().unwrap())
                .collect();

            (test, numbers)

        }).collect()

}

fn get_operators(n: usize) -> Vec<Vec<char>> {
    let combinations: Vec<Vec<char>> = (0..n-1).map(|_| OPERATORS).multi_cartesian_product().collect();
    combinations
}

fn apply_operators(test: isize, numbers: Vec<isize>) -> bool {
    let operators = get_operators(numbers.len());

    let result = operators
        .iter()
        .any(|seq| {
            let mut s = seq.iter();

            let result = numbers
                .iter()
                .copied()
                .reduce(|acc, next_number| match s.next().unwrap() {
                    '*' => acc * next_number,
                    '+' => acc + next_number,
                    _ => panic!("invalid operator"),
                }).unwrap();

            test == result
        });
    
    result
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    
    let file_path = &args[1];

    let input = utils::read_input(file_path);

    let parsed = parse_input(&input);


    let sum: isize = parsed
        .iter()
        .filter(|(test, numbers)| apply_operators(*test, numbers.clone()))
        .map(|(test, _)| *test)
        .sum();

    println!("Possible Equation sum: {sum}");

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

        // Parse the input into (test, numbers) pairs
        let parsed = parse_input(input);

        // Count how many equations return true
        let sum: isize = parsed
            .iter()
            .filter(|(test, numbers)| apply_operators(*test, numbers.clone()))
            .map(|(test, _)| *test)
            .sum();

        assert_eq!(sum, 3749);
    }


}
