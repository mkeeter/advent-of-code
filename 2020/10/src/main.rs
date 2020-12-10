use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let mut p: Vec<u64> = std::io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    p.push(0); // the wall outlet
    p.sort();
    p.push(p.last().unwrap() + 3); // our device

    let count = p.iter().zip(p.iter().skip(1))
        .map(|(a, b)| b - a)
        .fold((0, 0), |(n1, n3), d| {
            match d {
                1 => (n1 + 1, n3),
                2 => (n1, n3),
                3 => (n1, n3 + 1),
                _ => panic!("Cannot adapt!"),
            }});
    println!("Part 1: {}", count.0 * count.1);

    let adapters: HashSet<u64> = p.into_iter().collect();



}

