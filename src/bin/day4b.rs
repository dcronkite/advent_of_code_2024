use advent_of_code_2024::utils::readfile;
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
    // solve: look for A in middle of X, so cannot be on the edge
    let mut total_found: i32 = 0;
    for i in 1..puzzle.len() - 1 {
        for j in 1..puzzle[i].len() - 1 {
            if puzzle[i][j] == 'A' {
                total_found += start_search(&puzzle, i, j);
            }
        }
    }
    println!("Total found: {}", total_found);
    Ok(())
}

fn start_search(puzzle: &Vec<Vec<char>>, i: usize, j: usize) -> i32 {
    let mut count = 0;
    if puzzle[i - 1][j - 1] == 'M' && puzzle[i + 1][j + 1] == 'S' {
        count += 1;
    }
    if puzzle[i + 1][j + 1] == 'M' && puzzle[i - 1][j - 1] == 'S' {
        count += 1;
    }
    if puzzle[i + 1][j - 1] == 'M' && puzzle[i - 1][j + 1] == 'S' {
        count += 1;
    }
    if puzzle[i - 1][j + 1] == 'M' && puzzle[i + 1][j - 1] == 'S' {
        count += 1;
    }
    if count == 2 {
        1
    } else {
        0
    }
}