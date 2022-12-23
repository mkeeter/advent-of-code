use anyhow::Result;
use std::{
    collections::{BTreeMap, BTreeSet},
    io::BufRead,
};

fn main() -> Result<()> {
    let mut elves = BTreeSet::new();
    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                elves.insert((x as i64, y as i64));
            }
        }
    }

    const DIRS: [(i64, i64); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    for round in 0.. {
        let dir_iter = std::iter::repeat(DIRS).flatten().skip(round).take(4);
        let mut proposals = BTreeMap::new();
        for elf in &elves {
            // Check for neighbors
            let any_nearby = (-1..=1)
                .flat_map(|x| (-1..=1).map(move |y| (x, y)))
                .filter(|p| *p != (0, 0))
                .any(|(x, y)| elves.contains(&(elf.0 + x, elf.1 + y)));
            if !any_nearby {
                continue;
            }

            for dir in dir_iter.clone() {
                let any_nearby = (-1..=1).any(|i| {
                    elves.contains(&(
                        elf.0 + dir.0 + i * dir.1,
                        elf.1 + dir.1 + i * dir.0,
                    ))
                });
                if !any_nearby {
                    proposals.insert(*elf, (elf.0 + dir.0, elf.1 + dir.1));
                    break;
                }
            }
        }
        if proposals.is_empty() {
            println!("Part 2: {}", round + 1);
            break;
        }
        let mut proposal_count = BTreeMap::new();
        for p in proposals.values() {
            *proposal_count.entry(*p).or_insert(0) += 1;
        }
        for (start, end) in proposals.into_iter() {
            if proposal_count[&end] == 1 {
                elves.remove(&start);
                elves.insert(end);
            }
        }

        if round + 1 == 10 {
            let xmin = elves.iter().map(|p| p.0).min().unwrap_or(0);
            let xmax = elves.iter().map(|p| p.0).max().unwrap_or(0);
            let ymin = elves.iter().map(|p| p.1).min().unwrap_or(0);
            let ymax = elves.iter().map(|p| p.1).max().unwrap_or(0);

            let mut ground_tiles = 0;
            for y in ymin..=ymax {
                for x in xmin..=xmax {
                    if !elves.contains(&(x, y)) {
                        ground_tiles += 1;
                    }
                }
            }
            println!("Part 1: {ground_tiles}");
        }
    }

    Ok(())
}
