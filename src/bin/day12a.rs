use advent_of_code_2024::utils::readfile;
use std::collections::HashSet;
use std::hash::Hash;
use std::io;

fn main() -> io::Result<()> {
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in readfile("data/day12.txt") {
        let mut row: Vec<char> = Vec::new();
        for ch in line.chars() {
            row.push(ch);
        }
        map.push(row);
    }

    let mut cost: usize = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let ch: char = map[i][j];
            if visited.contains(&(i, j)) {
                continue;  // already visited
            } else {
                let (count, boundaries) = search(ch, i, j, &map, &mut visited);
                cost += count * boundaries;
            }
        }
    }
    println!("Total cost: {}", cost);

    Ok(())
}

fn search(
    ch: char,
    i: usize,
    j: usize,
    map: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    visited.insert((i, j));
    let mut count: usize = 1;
    let mut boundaries: usize = 0;
    if i == 0 {
        boundaries += 1;
    } else if map[i - 1][j] != ch {
        boundaries += 1;
    } else if visited.contains(&(i -1, j)) {
        // do nothing
    } else {
        let (_count, _boundaries) = search(ch, i-1, j, &map, visited);
        count += _count;
        boundaries += _boundaries;
    }
    if i == map.len() - 1 {
        boundaries += 1;
    } else if map[i + 1][j] != ch {
        boundaries += 1;
    } else if visited.contains(&(i +1, j)) {

    } else {
        let (_count, _boundaries) = search(ch, i+1, j, &map, visited);
        count += _count;
        boundaries += _boundaries;
    }
    if j == 0 {
        boundaries += 1;
    } else if map[i][j - 1] != ch {
        boundaries += 1;
    } else if visited.contains(&(i, j - 1)) {
        // do nothing: already visited
    } else {
        let (_count, _boundaries) = search(ch, i, j - 1, &map, visited);
        count += _count;
        boundaries += _boundaries;
    }
    if j == map[i].len() - 1 {
        boundaries += 1;
    } else if map[i][j + 1] != ch {
        boundaries += 1;
    } else if visited.contains(&(i, j + 1)) {

    } else {
        let (_count, _boundaries) = search(ch, i, j + 1, &map, visited);
        count += _count;
        boundaries += _boundaries;
    }

    (count, boundaries)
}
