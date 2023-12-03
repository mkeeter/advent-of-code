use anyhow::Result;
use std::{
    collections::{BTreeMap, BTreeSet},
    io::BufRead,
};

fn part1(
    numbers: &BTreeMap<(i64, i64), u32>,
    symbols: &BTreeMap<(i64, i64), char>,
) -> u32 {
    // Find the 3x3 neighborhood around each symbol
    let mut ns = BTreeSet::new();
    for (x, y) in symbols.keys() {
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                ns.insert((x + dx, y + dy));
            }
        }
    }
    // Find any numbers which overlap with those neighborhoods
    let mut out = 0;
    for ((x, y), v) in numbers {
        if (0..=v.ilog10()).any(|dx| ns.contains(&(*x + dx as i64, *y))) {
            out += v;
        }
    }
    out
}

fn part2(
    numbers: &BTreeMap<(i64, i64), u32>,
    symbols: &BTreeMap<(i64, i64), char>,
) -> u32 {
    // Build a map from (x, y) -> index of number in `nums`
    let mut numspan = BTreeMap::new();
    let mut nums = vec![];
    for (i, ((x, y), v)) in numbers.iter().enumerate() {
        for dx in 0..=v.ilog10() as i64 {
            numspan.insert((*x + dx, *y), i);
        }
        nums.push(*v);
    }

    let mut out = 0;
    for ((x, y), _) in symbols.iter().filter(|(_, c)| **c == '*') {
        // Find numbers (by index) which are neighbors of this gear
        let mut ns = BTreeSet::new();
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                if let Some(i) = numspan.get(&(x + dx, y + dy)) {
                    ns.insert(*i);
                }
            }
        }
        if ns.len() == 2 {
            out += ns.iter().map(|i| nums[*i]).product::<u32>();
        }
    }
    out
}

fn main() -> Result<()> {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<String>, _>>()?;

    let mut numbers = BTreeMap::new();
    let mut symbols = BTreeMap::new();
    for (y, line) in lines.iter().enumerate() {
        let mut number = None;
        for (x, c) in line.chars().enumerate() {
            if let Some(d) = c.to_digit(10) {
                let (pos, prev) = number.unwrap_or(((x as i64, y as i64), 0));
                number = Some((pos, prev * 10 + d));
            } else {
                if let Some((pos, v)) = number.take() {
                    numbers.insert(pos, v);
                }
                if c != '.' {
                    symbols.insert((x as i64, y as i64), c);
                }
            }
        }
        if let Some((pos, v)) = number.take() {
            numbers.insert(pos, v);
        }
    }

    println!("Part 1: {}", part1(&numbers, &symbols));
    println!("Part 2: {}", part2(&numbers, &symbols));
    Ok(())
}
