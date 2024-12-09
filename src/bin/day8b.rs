use advent_of_code_2024::utils::readfile;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::io;

/**
    Plan:
1. Count all '#' as antinodes, no need to track
2. Collect all letters/numbers as (x, y) coordinate system
3. Iterate through all combos of (2):
    a. create a line
    b. calculate distance between points
    c. calculate antinode location
    d. check if any antennae double as antinodes
    e. remove any found value to avoid double-counting location
*/

fn main() -> io::Result<()> {
    let mut antinode_locations: HashSet<(usize, usize)> = HashSet::new();
    let mut antenna_locations: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    let mut antennae_by_location: HashMap<(usize, usize), char> = HashMap::new();
    let mut length_x = 0;
    let mut length_y = 0;
    for (y, line) in readfile("data/day8.txt").enumerate() {
        length_x = line.len();
        for (x, ch) in line.char_indices() {
            match ch {
                '.' => continue,
                '#' => {
                    antinode_locations.insert((x, y));
                }
                _ => {
                    antinode_locations.insert((x, y));
                    antenna_locations
                        .entry(ch)
                        .or_insert_with(Vec::new)
                        .push((x, y));
                    antennae_by_location.insert((x, y), ch);
                }
            }
        }
        length_y = y + 1;
    }

    // solve
    println!(
        "Designated antinode locations: {}",
        antinode_locations.len()
    );
    for (ch, (x, y)) in get_expected_antinode_positions(antenna_locations, length_x, length_y) {
        antinode_locations.insert((x, y));
    }
    println!("Number of antinode locations: {}", antinode_locations.len());

    Ok(())
}

fn get_expected_antinode_positions(
    antenna_locations: HashMap<char, Vec<(usize, usize)>>,
    length_x: usize,
    length_y: usize,
) -> impl Iterator<Item = (char, (usize, usize))> {
    antenna_locations.into_iter().flat_map(move |(key, values)| {
        values.into_iter().combinations(2).flat_map(move |combo| {
            get_line_elements(combo[0], combo[1], length_x as i32, length_y as i32).map(move |point| (key, point))
        })
    })
}

fn get_line_elements(
    p1: (usize, usize),
    p2: (usize, usize),
    length_x: i32,
    length_y: i32,
) -> impl Iterator<Item = (usize, usize)> {
    let p1 = (p1.0 as i32, p1.1 as i32);
    let p2 = (p2.0 as i32, p2.1 as i32);
    let mut results: Vec<(i32, i32)> = Vec::new();
    let rise = p1.1 - p2.1; 
    let run = p1.0 - p2.0;
    let mut p0_y: i32 = p2.1 - rise;
    let mut p0_x: i32 = p2.0 - run;
    let mut p3_y: i32 = p1.1 + rise;
    let mut p3_x: i32 = p1.0 + run;
    while p0_x >= 0 && p0_x < length_x && p0_y >= 0 && p0_y < length_y {
        results.push((p0_x, p0_y));
        p0_y -= rise;
        p0_x -= run;
    }
    while p3_x >= 0 && p3_x < length_x && p3_y >= 0 && p3_y < length_y {
        results.push((p3_x, p3_y));
        p3_y += rise;
        p3_x += run;
    }
    results
        .into_iter()
        .filter_map(move |(x, y)| {
            if x >= 0 && y >= 0 && x < length_x && y < length_y{
                Some((x as usize, y as usize))
            } else {
                None
            }
        })
}
