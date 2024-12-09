use advent_of_code_2024::utils::readfile;
use std::io;

fn main() -> io::Result<()> {
    let mut uncompressed: Vec<i32> = Vec::new();
    let mut id: i32 = 0;
    for line in readfile("data/day9.txt") {
        for (i, ch) in line.char_indices() {
            let value = ch.to_digit(10).unwrap();
            for _ in 0..value {
                if i % 2 == 0 {  // file
                    uncompressed.push(id);
                } else {
                    // empty space
                    uncompressed.push(-1); // use -1 as sentinel value
                }
            }
            if i % 2 == 0 {
                id += 1;
            }
        }
    }

    let mut i: usize = 0;
    let mut j: usize = uncompressed.len() - 1;
    while i < j {
        if uncompressed[j] == -1 {
            j -= 1;
        } else if uncompressed[i] == -1 {
            uncompressed[i] = uncompressed[j];
            uncompressed[j] = -1;
            i += 1;
            j -= 1;
        } else {
            i += 1;
        }
    }

    // checksum
    let checksum: i128 = uncompressed
        .iter()
        .enumerate()
        .filter_map(
            |(i, &val)| {
                if val >= 0 {
                    Some(((i as i32) * val) as i128)
                } else {
                    None
                }
            },
        )
        .sum();

    println!("Checksum {checksum}");
    Ok(())
}
