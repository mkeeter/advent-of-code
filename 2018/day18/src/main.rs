use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Hash, Eq, PartialEq, Clone)]
struct Grid(Vec<Vec<char>>);

impl Grid {
    fn get(&self, x: i32, y: i32) -> Option<char> {
        if x < 0 || y < 0 {
            None
        } else {
            self.0.get(y as usize)?.get(x as usize).cloned()
        }
    }

    fn count(&self, r: char) -> usize {
        self.0
            .iter()
            .flat_map(|i| i.iter())
            .filter(|c| **c == r)
            .count()
    }

    fn step(&mut self) {
        let mut next = vec![vec!['.'; self.0.len()]; self.0.len()];
        for y in 0..self.0.len() {
            for x in 0..self.0.len() {
                let mut trees = 0;
                let mut yards = 0;
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }
                        if let Some(c) = self.get(x as i32 + i, y as i32 + j) {
                            match c {
                                '|' => trees += 1,
                                '#' => yards += 1,
                                '.' => continue,
                                _ => unimplemented!(),
                            }
                        }
                    }
                }
                next[y][x] = match self.0[y][x] {
                    '.' => {
                        if trees >= 3 {
                            '|'
                        } else {
                            '.'
                        }
                    }
                    '|' => {
                        if yards >= 3 {
                            '#'
                        } else {
                            '|'
                        }
                    }
                    '#' => {
                        if trees >= 1 && yards >= 1 {
                            '#'
                        } else {
                            '.'
                        }
                    }
                    _ => unimplemented!(),
                };
            }
        }
        self.0 = next;
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut grid = Grid(buffer.lines().map(|c| c.chars().collect()).collect());

    const N: usize = 1000000000;
    let mut seen = HashMap::new();
    let mut cycle = None;
    for i in 0..N {
        if let Some(j) = seen.get(&grid) {
            println!("Found cycle from {} to {}", j, i);
            cycle = Some((*j, i - j));
            break;
        } else {
            seen.insert(grid.clone(), i);
        }
        grid.step();
    }

    let (mut start, delta) = cycle.unwrap();
    start += delta * ((N - start) / delta);

    for _i in start..N {
        grid.step();
    }

    let yards = grid.count('#');
    let trees = grid.count('|');
    println!("{} x {} = {}", trees, yards, trees * yards);
}
