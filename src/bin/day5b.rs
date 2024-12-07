use std::collections::{HashMap, HashSet};
use std::io;
use advent_of_code_2024::utils::readfile;

fn main() -> io::Result<()>{
    let mut result = 0;
    let mut incorrect: Vec<Vec<i32>> = Vec::new();
    let mut line_iter = readfile("data/day5.txt");
    let mut rules : HashMap<i32, Vec<i32>> = HashMap::new();  // read as X requires Y {X: Y}
    while let Some(line) = line_iter.next() {
        if line.is_empty() {
            break;
        }
        let (num1s, num2s) = line.split_once("|").unwrap();
        let num1: i32 = num1s.trim().parse().unwrap();
        let num2: i32 = num2s.trim().parse().unwrap();
        rules.entry(num2).or_insert_with(Vec::new).push(num1);
    }
    for line in line_iter {
        let mut correct = true;
        let mut previous: HashSet<i32> = HashSet::new();
        let mut current_row: Vec<i32> = Vec::new();
        for value in line.split(",") {
            let number: i32 = value.parse().unwrap();
            current_row.push(number);
        }
        for number in &current_row {
            for prereq in rules.get(&number).unwrap() {
                if current_row.contains(prereq) && !previous.contains(prereq) {
                    correct = false;
                    break;
                }
            }
            previous.insert(*number);
            if !correct {
                break;
            }
        }
        if correct {
            result += current_row[current_row.len() / 2];
        } else {
            incorrect.push(current_row);
        }
    }
    println!("Total middle values: {}", result);

    let mut fixed_result = 0;
    for mut list in incorrect {
        let mut i = 0;
        let mut found : Vec<i32> = Vec::new();
        while ! list.is_empty() {
            if i >= list.len() {
                i = 0;
            }
            let curr = list[i];
            let mut has_prereq = false;
            for prereq in rules.get(&curr).unwrap() {
                if list.contains(prereq) && !found.contains(prereq) {
                    has_prereq = true;
                    break;
                }
            }
            if has_prereq {
                i += 1
            } else {
                found.push(curr);
                list.retain(|x| *x != curr);  // remove element from list
            }
        }
        fixed_result += found[found.len() / 2];
    }

    println!("Fixed result: {}", fixed_result);
    Ok(())
}