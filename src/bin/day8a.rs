use advent_of_code_2024::utils::readfile;
use std::collections::{HashMap, HashSet};
use std::io;
use itertools::Itertools;

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
    println!("Designated antinode locations: {}", antinode_locations.len());
    for (ch, (x, y)) in get_expected_antinode_positions(antenna_locations) {
        println!("{} ({}, {})", ch, x, y);
        // 348
        if x < length_x && y < length_y {
            antinode_locations.insert((x, y));
        }
        // if antennae_by_location.contains_key(&(x, y)) {
        //     if antennae_by_location.get(&(x, y)).unwrap() == &ch {
        //         continue;  // skip where the character is the same
        //     } else {
        //         antinode_locations.insert((x, y));
        //     }
        // }
    }
    println!("Number of antinode locations: {}", antinode_locations.len());

    Ok(())
}

fn get_expected_antinode_positions(
    antenna_locations: HashMap<char, Vec<(usize, usize)>>,
) -> impl Iterator<Item=(char, (usize, usize))> {
    antenna_locations.into_iter().flat_map(|(key, values)| {
        values.into_iter().combinations(2).flat_map(move |combo| {
            get_line_elements(combo[0], combo[1]).map(move |point| (key, point))
        })
    })
}

fn get_line_elements(p1: (usize, usize), p2: (usize, usize)) -> impl Iterator<Item=(usize, usize)> {
    let p1 = (p1.0 as i32, p1.1 as i32);
    let p2 = (p2.0 as i32, p2.1 as i32);
    let p0_y: i32 = p2.1 - (p1.1 - p2.1);
    let p0_x: i32 = p2.0 - (p1.0 - p2.0);
    let p3_y: i32 = p1.1 - (p2.1 - p1.1);
    let p3_x: i32 = p1.0 - (p2.0 - p1.0);
    println!("({}, {}), {:?}, {:?}, ({}, {})", p0_x, p0_y, p1, p2, p3_x, p3_y);
    [(p0_x, p0_y), (p3_x, p3_y)].into_iter().filter_map(|(x, y)| {
        if x >= 0 && y >= 0 {
            Some((x as usize, y as usize))
        } else {
            None
        }
    })
}
