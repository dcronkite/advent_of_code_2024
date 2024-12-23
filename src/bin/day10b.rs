use advent_of_code_2024::utils::readfile;
use std::io;

fn main() -> io::Result<()> {
    let mut map: Vec<Vec<usize>> = Vec::new();
    let mut zeros: Vec<(usize, usize)> = Vec::new(); // collect starting places
                                                     // build topography
    for (i, line) in readfile("data/day10.txt").enumerate() {
        let mut curr: Vec<usize> = Vec::new();
        for (j, ch) in line.char_indices() {
            curr.push(ch.to_digit(10).unwrap() as usize);
            if ch == '0' {
                zeros.push((i, j));
            }
        }
        map.push(curr);
    }
    // solve
    println!("Found {} trailheads.", zeros.len());
    let mut count : usize = 0;
    for (x, y) in zeros {
        let vec = find_trails(&map, x, y, 0);
        count += vec.len();
    }

    println!("Total score of all trails: {}", count);

    Ok(())
}

fn find_trails(map: &Vec<Vec<usize>>, x: usize, y: usize, elevation: usize) -> Vec<(usize, usize)> {
    let mut trails: Vec<(usize, usize)> = Vec::new();
    if elevation == 9 {
        trails.push((x, y));
        return trails;
    }
    let next_elevation: usize = elevation + 1;
    if x > 0 && map[x - 1][y] == next_elevation {
        trails.extend(&find_trails(&map, x - 1, y, next_elevation));
    }
    if x + 1 < map.len() && map[x + 1][y] == next_elevation {
        trails.extend(&find_trails(&map, x + 1, y, next_elevation));
    }
    if y > 0 && map[x][y - 1] == next_elevation {
        trails.extend(&find_trails(&map, x, y - 1, next_elevation));
    }
    if y + 1 < map[x].len() && map[x][y + 1] == next_elevation {
        trails.extend(&find_trails(&map, x, y + 1, next_elevation));
    }

    trails
}
