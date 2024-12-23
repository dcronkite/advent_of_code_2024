use advent_of_code_2024::utils::readfile;
use std::collections::HashSet;
use std::io;

fn main() -> io::Result<()> {
    let mut map: Vec<Vec<char>> = Vec::new();
    for line in readfile("data/day12.txt") {
        let mut row: Vec<char> = Vec::new();
        for ch in line.chars() {
            row.push(ch);
        }
        map.push(row);
    }

    let mut cost: usize = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            let ch: char = map[i][j];
            if visited.contains(&(i, j)) {
                continue; // already visited
            } else {
                let (count, edges) = search(ch, i, j, &map, &mut visited);
                cost += count * edges;
                println!(
                    "Cost for {}={} (#{} x {} sides)",
                    ch,
                    count * edges,
                    count,
                    edges
                );
            }
        }
    }
    println!("Total cost: {}", cost);

    Ok(())
}

struct BoundaryBox {
    min_i: usize,
    min_j: usize,
    max_i: usize,
    max_j: usize,
}
impl BoundaryBox {
    fn new() -> Self {
        BoundaryBox {
            min_i: 1000,
            min_j: 1000,
            max_i: 0,
            max_j: 0,
        }
    }

    fn zero(&mut self) -> bool {
        self.max_j - self.min_j == 0 && self.max_i - self.min_i == 0
    }

    fn add_i(&mut self, i: usize) {
        if i < self.min_i {
            self.min_i = i;
        }
        if i > self.max_i {
            self.max_i = i
        }
    }
    fn add_j(&mut self, j: usize) {
        if j < self.min_j {
            self.min_j = j;
        }
        if j > self.max_j {
            self.max_j = j
        }
    }
}

fn search(
    ch: char,
    i: usize,
    j: usize,
    map: &Vec<Vec<char>>,
    visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let mut count: usize = 1;
    visited.insert((i, j));

    let mut edges: usize = 0;

    // println!("({i}, {j}) {} = #{edges}", map[i][j]);
    // top left is corner
    if i == 0 && j == 0 {
        edges += 1;
    } else if i == 0 && map[i][j - 1] != ch {
        edges += 1;
    } else if j == 0 && map[i - 1][j] != ch {
        edges += 1;
    } else if i == 0 || j == 0 {
        // edge check
    } else if map[i][j - 1] != ch && map[i - 1][j] != ch {
        edges += 1;
    } else if map[i - 1][j - 1] != ch && map[i][j - 1] == ch && map[i - 1][j] == ch {
        edges += 1;
    }
    // println!("Top Left = #{edges}");
    // top right is corner
    if i == 0 && j == map[i].len() - 1 {
        edges += 1;
    } else if i == 0 && map[i][j + 1] != ch {
        edges += 1;
    } else if j == map[i].len() - 1 && map[i - 1][j] != ch {
        edges += 1;
    } else if i == 0 || j == map[i].len() - 1 {
        // edge check
    } else if map[i][j + 1] != ch && map[i - 1][j] != ch {
        edges += 1;
    } else if map[i - 1][j + 1] != ch && map[i][j + 1] == ch && map[i - 1][j] == ch {
        edges += 1;
    }
    // println!("Top Right = #{edges}");
    // bottom left is corner
    if i == map.len() - 1 && j == 0 {
        edges += 1;
    } else if i == map.len() - 1 && map[i][j - 1] != ch {
        edges += 1;
    } else if j == 0 && map[i + 1][j] != ch {
        edges += 1;
    } else if i == map.len() - 1 || j == 0 {
        // edge check
    } else if map[i][j - 1] != ch && map[i + 1][j] != ch {
        edges += 1;
    } else if map[i + 1][j - 1] != ch && map[i][j - 1] == ch && map[i + 1][j] == ch {
        edges += 1;
    }
    // println!("Bottom Left = #{edges}");
    // bottom right is corner
    if i == map.len() - 1 && j == map[i].len() - 1 {
        edges += 1;
    } else if i == map.len() - 1 && map[i][j + 1] != ch {
        edges += 1;
    } else if j == map[i].len() - 1 && map[i + 1][j] != ch {
        edges += 1;
    } else if i == map.len() - 1 || j == map[i].len() - 1 {
        // edge check
    } else if map[i][j + 1] != ch && map[i + 1][j] != ch {
        edges += 1;
    } else if map[i + 1][j + 1] != ch && map[i][j + 1] == ch && map[i + 1][j] == ch {
        edges += 1;
    }
    // println!("Bottom Right = #{edges}");

    // find next cell
    if i == 0 {
        // skip
    } else if map[i - 1][j] != ch {
        // edge
    } else if visited.contains(&(i - 1, j)) {
        // do nothing
    } else {
        let (_count, _edges) = search(ch, i - 1, j, &map, visited);
        count += _count;
        edges += _edges;
    }
    if i == map.len() - 1 {
        // skip
    } else if map[i + 1][j] != ch {
        // edge
    } else if visited.contains(&(i + 1, j)) {
        // already visited
    } else {
        let (_count, _edges) = search(ch, i + 1, j, &map, visited);
        count += _count;
        edges += _edges;
    }
    if j == 0 {
        //
    } else if map[i][j - 1] != ch {
        //
    } else if visited.contains(&(i, j - 1)) {
        // do nothing: already visited
    } else {
        let (_count, _edges) = search(ch, i, j - 1, &map, visited);
        count += _count;
        edges += _edges;
    }
    if j == map[i].len() - 1 {
        //
    } else if map[i][j + 1] != ch {
        //
    } else if visited.contains(&(i, j + 1)) {
        //
    } else {
        let (_count, _edges) = search(ch, i, j + 1, &map, visited);
        count += _count;
        edges += _edges;
    }

    (count, edges)
}
