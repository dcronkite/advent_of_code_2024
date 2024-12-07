use advent_of_code_2024::utils::readfile;
use std::collections::{HashSet};
use std::io;

fn main() -> io::Result<()> {
    // build map
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut i: usize = 0;
    let mut curr_i: usize = 0;
    let mut curr_j: usize = 0;
    let mut guard_state: char = '^';
    let guard_states: HashSet<char> = HashSet::from(['^', '>', '<', 'v']);
    for line in readfile("data/day6.txt") {
        let mut row: Vec<char> = Vec::new();
        for ch in line.chars() {
            if guard_states.contains(&ch) {
                curr_i = i;
                curr_j = row.len();
                guard_state = ch;
            };
            row.push(ch);
        }
        map.push(row);
        i += 1;
    }
    let length_i = map.len();
    let length_j = map[0].len();
    // solve puzzle
    let mut all_positions : HashSet<(usize, usize)> = HashSet::new();
    loop {
        println!("Current state: {} {} {} ({})", curr_i, curr_j, guard_state, map[curr_i][curr_j]);
        // add state
        all_positions.insert((curr_i, curr_j));
        match guard_state {
            '^' => {
                if curr_i == 0 { break; }
                if map[curr_i - 1][curr_j] == '#' {
                    guard_state = '>';
                } else {
                    curr_i -= 1;
                }
            }
            '>' => {
                if curr_j + 1 >= length_j { break; }
                if map[curr_i][curr_j + 1] == '#' {
                    guard_state = 'v';
                } else {
                    curr_j += 1;
                }
            }
            'v' => {
                if curr_i + 1 >= length_i { break; }
                if map[curr_i + 1][curr_j] == '#' {
                    guard_state = '<';
                } else {
                    curr_i += 1;
                }
            }
            '<' => {
                if curr_j == 0 { break; }
                if map[curr_i][curr_j - 1] == '#' {
                    guard_state = '^';
                } else {
                    curr_j -= 1;
                }
            }
            _ => {
                println!("Failed guard state: {}", guard_state);
                break;
            }
        }
    }

    println!("Guard visited {} distinct positions.", all_positions.len());
    Ok(())
}
