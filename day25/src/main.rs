#[macro_use] extern crate itertools;

use std::io::{self, Read};
use std::cmp::min;

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let pts = buffer.lines().map(
        |line| line.split(',').map(
            |word| word.parse::<i64>().unwrap())
        .collect::<Vec<_>>())
    .collect::<Vec<_>>();

    let edges = iproduct!(pts.iter().enumerate(),
                          pts.iter().enumerate())
        .filter(|((_, a), (_, b))|
                a.iter()
                 .zip(b.iter())
                 .map(|(a, b)| (a - b).abs())
                 .sum::<i64>() <= 3)
        .map(|((i, _), (j, _))| (i, j))
        .filter(|(i, j)| i != j)
        .collect::<Vec<_>>();

    let mut cliques: Vec<usize> = (0..pts.len()).collect();
    let mut changed = true;
    while changed {
        changed = false;
        for (i, j) in edges.iter() {
            if cliques[*i] != cliques[*j] {
                let c = min(cliques[*i], cliques[*j]);
                cliques[*i] = c;
                cliques[*j] = c;
                changed = true;
            }
        }
    }
    cliques.sort();
    cliques.dedup();
    println!("Part 1: {}", cliques.len());
    println!("Part 2: ❄");
}
