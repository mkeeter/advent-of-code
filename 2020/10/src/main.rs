use std::collections::{HashSet, HashMap};
use std::io::BufRead;

fn search(j: u8, cache: &mut HashMap<u8, u64>) -> u64 {
    if let Some(c) = cache.get(&j) {
        *c
    } else {
        let out = (1..=3)
            .filter_map(|i| j.checked_sub(i))
            .map(|j| search(j, cache))
            .sum();
        cache.insert(j, out);
        out
    }
}

fn main() {
    let mut p: Vec<u8> = std::io::stdin().lock().lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    p.push(0); // the wall outlet
    p.sort();
    let max = p.last().unwrap() + 3;
    p.push(max); // our device

    let count = p.iter().zip(p.iter().skip(1))
        .fold((0, 0), |(n1, n3), (a, b)| {
            match b - a {
                1 => (n1 + 1, n3),
                2 => (n1, n3),
                3 => (n1, n3 + 1),
                _ => panic!("Cannot adapt!"),
            }});
    println!("Part 1: {}", count.0 * count.1);

    let adapters: HashSet<u8> = p.into_iter().collect();

    // Cache how many ways we can get from a particular joltage to 0.
    // For all non-adapter joltages, the answer is 0, so pre-cache these.
    let mut cache: HashMap<u8, u64> = (0..max)
        .filter(|j| !adapters.contains(j))
        .map(|j| (j, 0))
        .collect();
    cache.insert(0, 1); // There is a trivial one way to get to joltage 0

    println!("Part 2: {}", search(max, &mut cache));
}
