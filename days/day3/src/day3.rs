use regex::Regex;

use std::env;

use utils::utils;

fn extract_mul(string: &str) -> i32 {
    let regex = Regex::new(r"(?:mul\()(\d+,\d+)(?:\))").unwrap();
    
    // result will be an iterator over tuples containing the start and end indices for each match in the string
    let result = regex.captures_iter(string);
    let mut total = 0;

    for mat in result {
      let mut parts= mat[1].split(",");
      if let (Some(a), Some(b)) = (parts.next(), parts.next()) {
        if let (Ok(x), Ok(y)) = (a.parse::<i32>(), b.parse::<i32>()) {
            let mul = x*y;
            total += mul;
        }
      }
    }

    total
}

fn part_2(string: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|(do|don't)\(\)").unwrap();
    let mut enabled = true;
    let mut sum = 0;

    let matches: Vec<_> = regex
        .find_iter(string)
        .collect();
    
    for mat in matches {
        let matched = mat.as_str();
        println!("{matched}");
        if matched == "do()" {
            enabled = true;
        } else if matched == "don't()" {
            enabled = false;
        } else if let Some(caps) = regex.captures(matched) {
            if enabled {
                let x: i32 = caps[1].parse().unwrap();
                let y: i32 = caps[2].parse().unwrap();
                sum += x * y;
            }
        }
    }
    sum

}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let file_path = &args[1];

    let input = utils::read_input(file_path);

    let result = extract_mul(&input);

    let part_2_result = part_2(&input);

    println!("Total: {result}");

    println!("Part 2 Total: {part_2_result}");
  }