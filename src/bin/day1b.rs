use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use counter::Counter;

fn main() -> io::Result<()> {
    let file = File::open("data/day1.txt")?;
    let reader = BufReader::new(file);
    let mut lefts : Vec<i32> = Vec::new();
    let mut rights : Counter<i32> = Counter::new();
    for line in reader.lines() {
        let line = line?;
        let pair: Vec<&str> = line.split_whitespace().collect();
        let left : i32 = pair[0].parse().expect("Expected int, got string.");
        lefts.push(left);
        let right: i32 = pair[1].parse().expect("Expected int, got string");
        rights[&right] += 1;
    }

    let mut score : i32 = 0;
    for value in lefts {
        let count = rights[&value];
        score += value * count as i32;
    }
    println!("Similarity score: {}", score);

    Ok(())
}