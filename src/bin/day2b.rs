use std::io;
use advent_of_code_2024::utils::readfile_as_ints;

fn main() -> io::Result<()>{
    let mut number_safe : i32 = 0;
    for row in readfile_as_ints("data/day2.txt") {
        let result = run_algorithm(&row);
        println!("Result {}: {:?}", result, row);
        if result < 2 {
            number_safe += 1;
            continue;
        }
        let result = run_algorithm(&row[1..].to_vec());
        if result < 1 {
            number_safe += 1;
            continue;
        }
        let result = run_algorithm(&[&row[..1], &row[2..]].concat().to_vec());
        if result < 1 {
            number_safe += 1;
            continue;
        }
    }
    println!("Number safe: {}", number_safe);
    Ok(())
}

fn run_algorithm(row: &Vec<i32>) -> i32 {
    let mut previous: i32 = row[0];
    let ascending;
    if previous < row[1] {
        ascending = true;
    } else if previous > row[1] {
        ascending = false;
    } else {
        return 100;  // equal
    }

    let mut safe = true;
    let mut num_errors = 0;
    for el in &row[1..] {
        if (previous - el).abs() > 3 {
            safe = false;
        }
        if ascending && previous >= *el {
            safe = false;
        }
        if !ascending && previous <= *el {
            safe = false;
        }
        if ! safe {
            num_errors += 1;
            safe = true;
        } else if safe {
            previous = *el;
        }
    }
    num_errors
}