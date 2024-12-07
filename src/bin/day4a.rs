use advent_of_code_2024::utils::readfile;
use itertools::Itertools;
use std::cmp::max;
use std::io;

fn main() -> io::Result<()> {
    // build puzzle
    let mut puzzle: Vec<Vec<char>> = Vec::new();
    let mut maxlength: usize = 0;
    for line in readfile("data/day4.txt") {
        let mut vec_row: Vec<char> = Vec::new();
        for ch in line.chars() {
            vec_row.push(ch);
        }
        maxlength = max(maxlength, vec_row.len());
        puzzle.push(vec_row);
    }
    // solve
    let mut total_found: i32 = 0;
    for i in 0..puzzle.len() {
        for j in 0..puzzle[i].len() {
            if puzzle[i][j] == 'X' {
                total_found += start_search(&puzzle, i as i32, j as i32);
            }
        }
    }
    println!("Total found: {}", total_found);
    Ok(())
}

fn start_search(puzzle: &Vec<Vec<char>>, i: i32, j: i32) -> i32 {
    let mut total = 0;
    let options = [-1, 0, 1];
    for (direction_i, direction_j) in options.iter().cartesian_product(options.iter()) {
        total += search(puzzle, i, j, *direction_i, *direction_j, Vec::from(['M', 'A', 'S']));
    }
    total
}
fn search(
    puzzle: &Vec<Vec<char>>,
    i: i32,
    j: i32,
    direction_i: i32,
    direction_j: i32,
    letters: Vec<char>,
) -> i32 {
    let mut curr_i = i;
    let mut curr_j = j;
    for letter in letters {
        curr_i += direction_i;
        curr_j += direction_j;
        // check bounds
        if curr_i < 0 || curr_i >= puzzle.len() as i32 {
            return 0;
        } else if curr_j < 0 || curr_j >= puzzle[curr_i as usize].len() as i32 {
            return 0;
        }
        if puzzle[curr_i as usize][curr_j as usize] != letter {
            return 0;
        }
    }
    1
}
