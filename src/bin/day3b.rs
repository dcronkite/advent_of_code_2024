use std::io;
use advent_of_code_2024::utils::readfile;
use regex::Regex;

fn main() -> io::Result<()> {
    let pattern = Regex::new(r"(mul|do|don't)\((\d{0,3}),?(\d{0,3})\)").unwrap();
    let mut result : i32 = 0;
    let mut do_operation = true;
    for row in readfile("data/day3.txt") {
        for (_, [operation, num1s, num2s]) in pattern.captures_iter(&row).map(|c| c.extract()) {
            let mut num1 :i32 = 0;
            let mut num2: i32 = 0;
            if num1s.len() > 0 && num2s.len() > 0 && operation != "do" && operation != "don't" {
                num1 = num1s.parse().unwrap();
                num2 = num2s.parse().unwrap();
            }
            match operation {
                "mul" => {if do_operation {result += num1 * num2}},
                "add" => {if do_operation {result += num1 + num2}},
                "do" => {do_operation = true;}
                "don't" => {do_operation = false;}
                _ => {}
            }
        }
    }
    println!("Total count: {}", result);

    Ok(())
}