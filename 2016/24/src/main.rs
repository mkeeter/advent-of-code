use std::collections::{HashMap, HashSet, VecDeque};
use std::io::BufRead;

use itertools::*;

fn search(
    tiles: &HashSet<(usize, usize)>,
    targets: &HashMap<(usize, usize), char>,
    start: (usize, usize),
) -> HashMap<char, usize> {
    let mut todo = VecDeque::new();
    let mut seen = HashSet::new();

    let mut out = HashMap::new();

    todo.push_back((start.0, start.1, 0));
    while let Some((x, y, steps)) = todo.pop_front() {
        if !seen.insert((x, y)) || !tiles.contains(&(x, y)) {
            continue;
        }

        if let Some(c) = targets.get(&(x, y)) {
            out.insert(*c, steps);
        }

        todo.push_back((x + 1, y, steps + 1));
        todo.push_back((x - 1, y, steps + 1));
        todo.push_back((x, y + 1, steps + 1));
        todo.push_back((x, y - 1, steps + 1));
    }
    out
}

fn main() {
    let mut tiles = HashSet::new();
    let mut targets = HashMap::new();
    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            if c != '#' {
                tiles.insert((x, y));
                if c != '.' {
                    targets.insert((x, y), c);
                }
            }
        }
    }

    let mut distances: HashMap<(char, char), usize> = HashMap::new();
    for (pos, c) in targets.iter() {
        for (t, d) in search(&tiles, &targets, *pos) {
            if t == *c {
                continue;
            } else {
                distances.insert((*c, t), d);
            }
        }
    }

    let p1 = targets
        .values()
        .filter(|c| **c != '0')
        .permutations(targets.len() - 1)
        .map(|s| {
            distances[&('0', *s[0])]
                + s.windows(2)
                    .map(|w| distances[&(*w[0], *w[1])])
                    .sum::<usize>()
        })
        .min()
        .unwrap();
    println!("Part 1: {}", p1);

    let p2 = targets
        .values()
        .filter(|c| **c != '0')
        .permutations(targets.len() - 1)
        .map(|s| {
            distances[&('0', *s[0])]
                + s.windows(2)
                    .map(|w| distances[&(*w[0], *w[1])])
                    .sum::<usize>()
                + distances[&(*s[s.len() - 1], '0')]
        })
        .min()
        .unwrap();
    println!("Part 2: {}", p2);
}
