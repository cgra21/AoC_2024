use advent_of_code::utils;

use std::fs;
use std::env;

fn find_xmas(input: Vec<Vec<&str>>) -> i32 {

    let match_word = "XMAS";
    let directions: Vec<(isize, isize)> = utils::Vectors::all_directions();

    let grid_width: usize = input[0].len();
    let grid_height: usize = input.len();

    let mut matches = 0;

    for row_idx in 0..grid_height {
        for col_idx in 0..grid_width {
            if input[row_idx][col_idx] == "X" {
                for direction in directions {

                    let chars_left = match_word.len() - 1;
                    if !(0 <= (col_idx as isize + chars_left as isize  * direction.0)
                        && (col_idx as isize  + chars_left as isize  * direction.0) < grid_width as isize ) {
                        continue;
                    }
                    if !(0 <= (row_idx as isize  + chars_left as isize  * direction.1)
                        && (row_idx as isize  + chars_left as isize  * direction.1) < grid_height as isize) {
                        continue;
                    }
                    
                    let mut found_word = true;

                    for (steps, letter) in match_word[1..].chars().enumerate() {
                        let new_row_idx = row_idx as isize + steps as isize *direction.1;
                        let new_col_idx = row_idx as isize + steps as isize *direction.0;
                        if input[new_row_idx][new_col_idx] != letter {
                            found_word = false;
                            break;
                        }
                    }
                    
                    if found_word {
                        matches += 1;
                    }

                }
            }
        }
    }

    matches
}


fn main() {

}