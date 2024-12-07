use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

fn main() -> io::Result<()>  {
    let file = File::open("data/day1.txt")?;
    let reader = BufReader::new(file);

    let mut lefts : Vec<i32> = Vec::new();
    let mut rights : Vec<i32> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let segments: Vec<&str> = line.split_whitespace().collect();
        assert_eq!(segments.len(), 2);
        let left : i32 = segments[0].parse().expect("Failed to parse string as int.");
        let right : i32 = segments[1].parse().expect("Failed to parse string as int.");
        lefts.push(left);
        rights.push(right);
    }
    lefts.sort();
    rights.sort();
    let mut difference: i32 = 0;
    for (left, right) in lefts.iter().zip(rights.iter()) {
        let result: i32 = (left - right).abs();
        difference += result;
    }
    println!("Total difference: {}", difference);
    Ok(())
}