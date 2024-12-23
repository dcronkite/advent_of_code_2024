use std::io;
use advent_of_code_2024::utils::readfile;


fn clean_rock(rock: &str) -> String {
    let trimmed = rock.trim_start_matches('0');
    if trimmed.is_empty() {
        "0".to_string()
    } else {
        trimmed.to_string()
    }
}
fn main() -> io::Result<()> {
    let iterations: isize = 20;
    let binding = readfile("data/day11.txt").next().unwrap();
    let mut rocks : Vec<String> = binding.split_whitespace().map(String::from).collect();

    // solve
    for i in 0..iterations {
        println!("Rock State at {i}: Length {}: {:?}", &rocks.len(), rocks);
        // println!("Rock State at {i}: Length {}", &rocks.len());
        let mut new_rocks: Vec<String> = Vec::new();
        for rock in rocks {
            if rock == "0" {
                new_rocks.push("1".to_string());
            } else if rock.len() % 2 == 0 {
                let half = rock.len() / 2;
                new_rocks.push(rock[..half].to_string());
                new_rocks.push(clean_rock(&rock[half..]));
            } else {
                let int_rock: i128 = rock.parse().unwrap();
                let new_val : i128 = int_rock * 2024;
                new_rocks.push(new_val.to_string());
            }
        }
        rocks = new_rocks.clone();
    }
    println!("Rock State at {iterations}: Length {}", &rocks.len());
    Ok(())
}