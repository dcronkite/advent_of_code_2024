use advent_of_code_2024::utils::readfile;
use std::io;

fn main() -> io::Result<()> {
    let mut uncompressed: Vec<i32> = Vec::new();
    let mut id: i32 = 0;
    for line in readfile("data/day9.txt") {
        for (i, ch) in line.char_indices() {
            let value = ch.to_digit(10).unwrap();
            for _ in 0..value {
                if i % 2 == 0 {
                    // file
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
    let mut max_id: i32 = uncompressed.iter().max().unwrap().clone();
    let mut j: usize = uncompressed.len() - 1; // find blocks to move over
    while j >= 0 {
        if j > 0 && uncompressed[j] == -1 {
            j -= 1; // looking for blocks of files
        } else if j > 0 && uncompressed[j] > max_id {
            j -= 1; // already moved this file
        } else {
            // found suitable id
            max_id = uncompressed[j];
            let mut count: usize = 1; // found file block >= 1
            while j > 0 {
                j -= 1; // look at previous block
                if uncompressed[j] == max_id {
                    count += 1; // found more of file
                    continue;
                } else {
                    let mut i: usize = 0; // find space to move blocks into
                                          // look for block of missing
                    loop {
                        // find a block to move the file into
                        while i < j && uncompressed[i] != -1 {
                            i += 1;
                        }
                        if i >= j {
                            break;
                        }
                        // found at least 1 with available space
                        let mut missing_cnt: usize = 0;
                        // figure out how much free space is available
                        while uncompressed[i] == -1 {
                            i += 1;
                            missing_cnt += 1;
                        }
                        if missing_cnt >= count {
                            for k in (i - missing_cnt)..(i - missing_cnt + count) {
                                uncompressed[k] = max_id;
                            }
                            for k in (j + 1)..(j + count + 1) {
                                // zero-out values
                                uncompressed[k] = -1;
                            }
                            break;
                        } else {
                            continue; // continue look for available space
                        }
                    }
                    break;
                }
            }
            max_id -= 1;
        }
        if j == 0 {
            break;
        }
    }

    println!("{:?}", uncompressed);

    // checksum
    let checksum: i128 = uncompressed
        .iter()
        .enumerate()
        .filter_map(|(i, &val)| {
            if val >= 0 {
                Some(((i as i32) * val) as i128)
            } else {
                None
            }
        })
        .sum();

    println!("Checksum {checksum}");
    Ok(())
}
