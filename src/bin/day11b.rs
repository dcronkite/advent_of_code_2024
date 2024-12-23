use advent_of_code_2024::utils::readfile;
use counter::Counter;
use std::io;
use std::ops::Add;

fn clean_rock(rock: &str) -> String {
    let trimmed = rock.trim_start_matches('0');
    if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}
fn main() -> io::Result<()> {
    let iterations: isize = 75;
    let binding = readfile("data/day11.txt").next().unwrap();
    let mut rocks: Counter<String, i128> = binding.split_whitespace().map(String::from).collect();

    // solve
    for i in 0..iterations {
        // println!("Rock State at {i}: {:?}", rocks);
        let count: i128 = rocks.total();
        println!(
            "Rock State at {i}: Length: {}, Count {}",
            &rocks.len(),
            count
        );
        let mut new_rocks: Counter<String, i128> = Counter::new();
        for (rock, count) in rocks {
            if rock == "0" {
                new_rocks[&"1".to_string()] += count;
            } else if rock.len() % 2 == 0 {
                let half = rock.len() / 2;
                new_rocks[&rock[..half].to_string()] += count;
                new_rocks[&clean_rock(&rock[half..])] += count;
            } else {
                let int_rock: i128 = rock.parse().unwrap();
                let new_val: i128 = int_rock * 2024;
                new_rocks[&new_val.to_string()] += count;
            }
        }
        rocks = new_rocks.clone();
    }
    let count: i128 = rocks.total();
    println!(
        "Rock State at {iterations}: Length {} Count {}",
        &rocks.len(),
        count
    );
    Ok(())
}
