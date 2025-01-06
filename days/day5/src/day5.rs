use std::{collections::HashMap, env};
use std::cmp::Ordering;

use utils::utils;

fn parse_input(input: &str) -> Vec<&str>{

    let parts: Vec<&str> = input.split("\n\n").collect();

    assert!(parts.len() == 2, "Input is not properly formatted for splitting.");

    parts
}

fn generate_hashmap(rules: &str) -> HashMap<String, Vec<String>> {

    let mut map: HashMap<String, Vec<String>> = HashMap::new();

    for rule in rules.lines() {

        let (key, value) = rule.split_once('|').expect("Invalid rule");

        map.entry(key.to_string())
            .and_modify(|vec| vec.push(value.to_string()))
            .or_insert_with(|| vec![value.to_string()]);
    }

    map
}

fn find_correct_updates(updates: &str, mapping: &HashMap<String, Vec<String>>) -> usize {

    let updates = updates.lines();

    let mut total = 0;

    for update in updates {
        let mut seen_pages: Vec<&str> = Vec::new();
        let pages = update.split(',');

        let mut valid = true;

        for page in pages {
            if let Some(after_pages) = mapping.get(page) {
                if after_pages.iter().any(|p| seen_pages.contains(&p.as_str())) {
                    valid = false;
                    break;
                } 
            }
            seen_pages.push(page);
        }

        if valid {
            let middle_index = seen_pages.len() / 2;
            let middle_page = seen_pages[middle_index];

            total += middle_page.parse::<usize>().expect("Invalid page number");
        }
        

    }

    total
}

fn comparator(a: &str, b: &str, rules: &HashMap<String, Vec<String>>) -> Ordering {
    if let Some(after_pages) = rules.get(a) {
        if after_pages.contains(&b.to_string()) {
            return Ordering::Less; // a must come before b
        }
    }

    if let Some(after_pages) = rules.get(b) {
        if after_pages.contains(&a.to_string()) {
            return Ordering::Greater; // b must come before a
        }
    }

    Ordering::Equal // No rule specifies precedence
}

fn part_2(updates: &str, mapping: &HashMap<String, Vec<String>>) -> usize {
    let updates = updates.lines();

    let mut total = 0;

    for update in updates {
        let mut seen_pages: Vec<&str> = Vec::new();
        let mut pages: Vec<&str> = update.split(',').collect();

        let mut valid = true;

        for page in &pages {
            if let Some(after_pages) = mapping.get(*page) {
                if after_pages.iter().any(|p| seen_pages.contains(&p.as_str())) {
                    valid = false;
                    break;
                } 
            }
            seen_pages.push(page);
        }

        if !valid {
            pages.sort_by(|a, b| comparator(a, b, mapping));

            let middle_index = pages.len() / 2;
            let middle_page = pages[middle_index];
            total += middle_page.parse::<usize>().expect("Invalid page number");

        }
    }
    total
}   

fn main() {

    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let input = utils::read_input(file_path);

    let parts = parse_input(&input);

    let mapping = generate_hashmap(parts[0]);

    let total = find_correct_updates(parts[1], &mapping);

    let edit_total = part_2(parts[1], &mapping);

    println!("Part 1: {total:?}");

    println!("Part 2: {edit_total}");

}