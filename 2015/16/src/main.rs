use itertools::*;
use std::collections::HashMap;
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let sues = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .replace(|c| c == ',' || c == ':', "")
                .split(' ')
                .tuples()
                .map(|(a, b)| (a.to_string(), usize::from_str(b).unwrap()))
                .collect::<HashMap<String, usize>>()
        })
        .collect::<Vec<HashMap<String, usize>>>();

    let known = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .iter()
    .map(|(k, v)| ((*k).to_string(), *v))
    .collect::<HashMap<String, usize>>();

    'part1: for s in sues.iter() {
        for k in known.iter() {
            if let Some(q) = s.get(k.0) {
                if q != k.1 {
                    continue 'part1;
                }
            }
        }
        println!("Part 1: {}", s["Sue"]);
        break;
    }

    'part2: for s in sues.iter() {
        for k in known.iter() {
            if let Some(q) = s.get(k.0) {
                if k.0 == "cats" || k.0 == "trees" {
                    if q <= k.1 {
                        continue 'part2;
                    }
                } else if k.0 == "pomeranians" || k.0 == "goldfish" {
                    if q >= k.1 {
                        continue 'part2;
                    }
                } else if q != k.1 {
                    continue 'part2;
                }
            }
        }
        println!("Part 2: {}", s["Sue"]);
        break;
    }
}
