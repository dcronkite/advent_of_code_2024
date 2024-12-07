use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn readfile_as_ints(path: &str) -> impl Iterator<Item = Vec<i32>> {
    let file = File::open(path).expect("Failed to open file.");
    let reader = BufReader::new(file);
    reader.lines().filter_map(|line| {
        line.ok().map(|el| {
            el.split_whitespace().filter_map(|s| s.parse().ok()).collect()
        })
    })
}

pub fn readfile(path: &str) -> impl Iterator<Item = String> {
    let file = File::open(path).expect("Failed to open file.");
    let reader = BufReader::new(file);
    reader.lines().filter_map(|line| {
        line.ok()
    })
}