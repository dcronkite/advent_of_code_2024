use std::io;
use advent_of_code_2024::utils::readfile_as_ints;

fn main() -> io::Result<()>{
    let mut number_safe : i32 = 0;
    for row in readfile_as_ints("data/day2.txt") {
        let mut previous :i32 = row[0];
        let ascending;
        if previous < row[1] {
            ascending = true;
        } else if previous > row[1] {
            ascending = false;
        } else {
            continue;  // equal
        }


        let mut safe = true;
        for el in &row[1..] {
            if (previous - el).abs() > 3 {
                safe = false;
                break;
            }
            if ascending && previous >= *el {
                safe = false;
                break;
            }
            if !ascending && previous <= *el {
                safe = false;
                break;
            }
            previous = *el;
        }
        if safe {
            number_safe += 1;
        }
    }
    println!("Number safe: {}", number_safe);
    Ok(())
}