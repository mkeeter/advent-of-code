use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::str::FromStr;
use itertools::*;

fn main() {
    let input = std::io::stdin().lock()
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let sign = if line.contains("gain") { 1 } else { -1 };
            let a = line.split(" ").next().unwrap().to_owned();
            let mut b = line.split(" ").last().unwrap().to_owned();
            b.pop(); // Remove trailing '.'
            let v = line.split(" ")
                .filter_map(|i| i32::from_str(i).ok())
                .next()
                .unwrap();
            ((a, b), v * sign)
        })
        .collect::<HashMap<(String, String), i32>>();

    let names = input.keys()
        .map(|k| k.0.to_owned())
        .collect::<HashSet<String>>()
        .into_iter()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect::<HashMap<String, usize>>();
    let happiness = input.into_iter()
        .map(|((a, b), v)| ((names[&a], names[&b]), v))
        .collect::<HashMap<(usize, usize), i32>>();

    let best = (0..names.len())
        .permutations(names.len())
        .filter(|p| p[0] == 0)
        .map(|p| p.iter()
             .zip(p.iter().cycle().skip(1))
             .map(|(a, b)| happiness[&(*a,*b)] + happiness[&(*b,*a)])
             .sum::<i32>())
        .max()
        .unwrap();
    println!("Part 1: {}", best);

    let best = (0..=names.len())
        .permutations(names.len() + 1)
        .filter(|p| p[0] == 0)
        .map(|p| p.iter()
             .zip(p.iter().cycle().skip(1))
             .map(|(a, b)| happiness.get(&(*a, *b)).unwrap_or(&0) +
                           happiness.get(&(*b, *a)).unwrap_or(&0))
             .sum::<i32>())
        .max()
        .unwrap();
    println!("Part 2: {}", best);
}
