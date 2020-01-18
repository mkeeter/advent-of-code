use std::io::{self, Read};
use std::collections::HashSet;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let mut x : i32 = 0;
    let mut y : i32 = 0;
    let mut dir : i32 = 0;
    let vecs = [[0, 1], [-1, 0], [0, -1], [1, 0]];

    let mut visited = HashSet::new();
    visited.insert((0,0));

    let mut p2 = None;
    for cmd in buffer.split(',') {
        let trimmed = cmd.trim();
        dir = match trimmed.as_bytes()[0] as char {
            'R' => if dir == 0 { 3 } else { dir - 1 },
            'L' => if dir == 3 { 0 } else { dir + 1 },
            _ => panic!("OMG"),
        };

        let steps = trimmed[1..].parse::<usize>().unwrap();
        for _ in 0..steps {
            x += vecs[dir as usize][0];
            y += vecs[dir as usize][1];

            if !visited.insert((x, y)) && p2.is_none() {
                p2 = Some(x.abs() + y.abs());
            }
        }
    }
    println!("Part 1: {}", x.abs() + y.abs());
    println!("Part 2: {}", p2.unwrap());
}
