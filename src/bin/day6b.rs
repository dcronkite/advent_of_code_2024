use advent_of_code_2024::utils::readfile;
use std::collections::HashSet;
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

    // solve puzzle
    let mut obstacles : HashSet<(usize, usize)> = HashSet::new();
    let all_positions : Vec<(usize, usize, char)> = get_positions(&map, curr_i, curr_j, guard_state);
    let num_positions : usize = all_positions.len();
    println!("Number of positions: {}", num_positions);
    let mut i : usize = 0;
    for (next_i, next_j, next_state) in all_positions {
        let mut map_temp = map.clone();
        i += 1;
        println!("Looking at #{i}/{num_positions}: {next_i} {next_j} {next_state} (starts at {curr_i} {curr_j})");
        map_temp[next_i][next_j] = '#';
        if has_loop(map_temp, curr_i, curr_j, guard_state) {
            obstacles.insert((next_i, next_j));
        }
    }
    println!("Found {} loops.", obstacles.len());

    Ok(())
}

fn has_loop(
    map: Vec<Vec<char>>,
    mut curr_i: usize,
    mut curr_j: usize,
    mut guard_state: char,
) -> bool {
    let length_i = map.len();
    let length_j = map[0].len();
    let mut path : HashSet<(usize, usize, char)> = HashSet::new();
    // solve puzzle
    loop {
        // add state
        if path.len() > 4 {
            if path.contains(&(curr_i, curr_j, guard_state)) {
                return true;
            }
        }
        path.insert((curr_i, curr_j, guard_state));
        match guard_state {
            '^' => {
                if curr_i == 0 {
                    return false;
                }
                if map[curr_i - 1][curr_j] == '#' {
                    guard_state = '>';
                } else {
                    curr_i -= 1;
                }
            }
            '>' => {
                if curr_j + 1 >= length_j {
                    return false;
                }
                if map[curr_i][curr_j + 1] == '#' {
                    guard_state = 'v';
                } else {
                    curr_j += 1;
                }
            }
            'v' => {
                if curr_i + 1 >= length_i {
                    return false;
                }
                if map[curr_i + 1][curr_j] == '#' {
                    guard_state = '<';
                } else {
                    curr_i += 1;
                }
            }
            '<' => {
                if curr_j == 0 {
                    return false;
                }
                if map[curr_i][curr_j - 1] == '#' {
                    guard_state = '^';
                } else {
                    curr_j -= 1;
                }
            }
            _ => {
                println!("Failed guard state: {}", guard_state);
                return false;
            }
        }
    }
}
fn get_positions(
    map: &Vec<Vec<char>>,
    mut curr_i: usize,
    mut curr_j: usize,
    mut guard_state: char,
) ->  Vec<(usize, usize, char)> {
    let length_i = map.len();
    let length_j = map[0].len();
    let start_i = curr_i;
    let start_j = curr_j;
    // solve puzzle
    let mut all_positions: Vec<(usize, usize, char)> = Vec::new();
    loop {
        // add state
        match guard_state {
            '^' => {
                if curr_i == 0 {
                    break;
                }
                if map[curr_i - 1][curr_j] == '#' {
                    guard_state = '>';
                } else {
                    curr_i -= 1;
                }
            }
            '>' => {
                if curr_j + 1 >= length_j {
                    break;
                }
                if map[curr_i][curr_j + 1] == '#' {
                    guard_state = 'v';
                } else {
                    curr_j += 1;
                }
            }
            'v' => {
                if curr_i + 1 >= length_i {
                    break;
                }
                if map[curr_i + 1][curr_j] == '#' {
                    guard_state = '<';
                } else {
                    curr_i += 1;
                }
            }
            '<' => {
                if curr_j == 0 {
                    break;
                }
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
        if start_i != curr_i || start_j != curr_j {
            all_positions.push((curr_i, curr_j, guard_state));
        }
    }

    println!("Guard visited {} distinct positions.", all_positions.len());
    all_positions
}
