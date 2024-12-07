use advent_of_code_2024::utils::readfile;
use std::io;
use itertools::Itertools;

fn main() -> io::Result<()> {
    let chars = ['*', '+'];
    let mut sum : i128 = 0;
    for line in readfile("data/day7.txt") {
        let (result_s, operands_s) = line.split_once(':').unwrap();
        let result: i128 = result_s.trim().parse().unwrap();
        let digits: Vec<i128> = operands_s
            .trim()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        for combo in generate_permutations(&chars, digits.len() -1) {
            let mut res : i128 = digits[0];
            for (op, num) in combo.iter().zip(digits[1..].iter()) {
                match op {
                    '*' => res *= num,
                    '+' => res += num,
                    _ => {}
                }
            }
            if res == result {
                println!("Found {}: {:?}", result, combo);
                sum += res;
                break;
            }
        }
    }

    println!("Result: {}", sum);

    Ok(())
}


fn generate_combinations(chars: &[char], length: usize) -> Vec<Vec<char>> {
    chars.iter()
        .cloned()
        .combinations_with_replacement(length)
        .map(|v| v.into_iter().collect())
        .collect()
}
fn generate_permutations(chars: &[char], length: usize) -> Vec<Vec<char>> {
    std::iter::repeat(chars.iter())
        .take(length)
        .multi_cartesian_product()
        .map(|v| v.into_iter().cloned().collect())
        .collect()
}
