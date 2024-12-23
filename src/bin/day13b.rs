use advent_of_code_2024::utils::readfile;
use regex::{Captures, Regex};
use std::io;

use nalgebra::linalg::LU;
use nalgebra::{Matrix2, Vector2};

fn parse(results: Captures) -> (f64, f64) {
    let x = results.get(1).unwrap().as_str().parse::<f64>().unwrap();
    let y = results.get(2).unwrap().as_str().parse::<f64>().unwrap();
    (x, y)
}

fn main() -> io::Result<()> {
    let pattern = Regex::new(r"(?:Button [AB]|Prize): X[+=](\d+), Y[+=](\d+)").unwrap();
    let mut button_a_x: f64 = 0.;
    let mut button_a_y: f64 = 0.;
    let mut button_b_x: f64 = 0.;
    let mut button_b_y: f64 = 0.;
    let mut prize_x: f64 = 0.;
    let mut prize_y: f64 = 0.;
    let mut total_cost: i64 = 0;
    for (i, line) in readfile("data/day13.txt").enumerate() {
        match i % 4 {
            0 => {
                let results = pattern.captures(&line).unwrap();
                (button_a_x, button_a_y) = parse(results);
            }
            1 => {
                let results = pattern.captures(&line).unwrap();
                (button_b_x, button_b_y) = parse(results);
            }
            2 => {
                let results = pattern.captures(&line).unwrap();
                (prize_x, prize_y) = parse(results);
            }
            3 => {
                // do calculation
                total_cost += calculate(
                    [button_a_x, button_a_y],
                    [button_b_x, button_b_y],
                    prize_x + 10000000000000.,
                    prize_y + 10000000000000.,
                );

                println!("Cost: {}", total_cost);
            }
            _ => unreachable!(),
        }
    }
    println!("Total cost: {}", total_cost);

    Ok(())
}

fn calculate(a: [f64; 2], b: [f64; 2], prize_x: f64, prize_y: f64) -> i64 {
    let m = Matrix2::new(a[0], b[0], a[1], b[1]);
    let m2 = Vector2::new(prize_x, prize_y);
    let lu = LU::new(m);
    let res_m = lu.solve(&m2).unwrap();
    let res_a = res_m[0].round() as i64;
    let res_b = res_m[1].round() as i64;
    let x_result = res_a as f64 * a[0] + res_b as f64 * b[0];
    let y_result = res_a as f64 * a[1] + res_b as f64 * b[1];
    if x_result == prize_x && y_result == prize_y {
        res_a * 3 + res_b
    } else {
        0
    }
}
