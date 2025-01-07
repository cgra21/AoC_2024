use std::{collections::{HashMap, HashSet}, env};

use utils::utils;
use ::utils::utils::Point;

use itertools::Itertools;


fn find_antennas(grid: Vec<Vec<char>>) -> HashMap<char, HashSet<Point>> {

    let mut antennas: HashMap<char, HashSet<Point>> = HashMap::new();

    for (row_idx, row) in grid.iter().enumerate() {
        for (col_idx, &ch) in row.iter().enumerate() {
            if ch.is_alphanumeric() {
                let position = Point {
                    x: col_idx as isize,
                    y: row_idx as isize,
                };

                antennas
                    .entry(ch)
                    .or_insert_with(HashSet::new)
                    .insert(position);
            }
        }
    }

    antennas
}

fn antinodes(antennas: HashMap<char, HashSet<Point>>, rows: usize, cols: usize) -> HashSet<Point> {
    let mut antinodes: HashSet<Point> = HashSet::new();

    antennas.iter().for_each(|(_key, positions)| {
        positions.iter()
            .combinations(2)
            .for_each(|combo| {
                if let [&p1, &p2] = &combo[..] {
                    let (p1, p2) = if p1 < p2 { (p1, p2) } else { (p2, p1) };

                    let delta = p2 - p1;

                    // Do I need to insert p2, since current_antinode is just that?
                    antinodes.insert(p1);
                    antinodes.insert(p2);

                    // Generate all possible antinodes in both directions
                    // Forward direction
                    let mut current_antinode = p1 + delta;
                    while current_antinode.x >= 0 && current_antinode.x < cols as isize
                        && current_antinode.y >= 0 && current_antinode.y < rows as isize
                    {
                        antinodes.insert(current_antinode);
                        current_antinode = current_antinode + delta;
                    }

                    // Backward direction
                    let mut current_antinode = p1 - delta;
                    while current_antinode.x >= 0 && current_antinode.x < cols as isize
                        && current_antinode.y >= 0 && current_antinode.y < rows as isize
                    {
                        antinodes.insert(current_antinode);
                        current_antinode = current_antinode - delta;
                    }
                }
            });
    });

    antinodes
}



fn main() {

    let args: Vec<String> = env::args().collect();
    
    let file_path = &args[1];

    let input = utils::read_input(file_path);

    let grid = utils::parse_grid(&input);
    let rows = grid.len();
    let cols =  grid[0].len();

    let antennas = find_antennas(grid.clone());

    let antinodes = antinodes(antennas, rows, cols).len();

    println!("Antinodes: {antinodes:?}");

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_antenna() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let grid = utils::parse_grid(&input);

        let antennas = find_antennas(grid);

        assert_eq!(antennas.len(), 2);

        // Assert the size of each character's HashSet
        assert_eq!(antennas.get(&'0').map(|set| set.len()), Some(4)); // Expecting 4 positions for '0'
        assert_eq!(antennas.get(&'A').map(|set| set.len()), Some(3)); // Expecting 3 positions for 'A'
    }
}