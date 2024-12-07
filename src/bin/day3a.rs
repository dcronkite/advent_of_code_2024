use std::io;
use advent_of_code_2024::utils::readfile;
use regex::Regex;

fn main() -> io::Result<()> {
    let pattern = Regex::new(r"(mul)\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut result : i32 = 0;
    for row in readfile("data/day3.txt") {
        for (_, [operation, num1s, num2s]) in pattern.captures_iter(&row).map(|c| c.extract()) {
            let num1 : i32 = num1s.parse().unwrap();
            let num2 : i32 = num2s.parse().unwrap();
            match operation {
                "mul" => {result += num1 * num2},
                "add" => {result += num1 + num2},
                _ => {}
            }
        }
    }
    println!("Total count: {}", result);

    Ok(())
}