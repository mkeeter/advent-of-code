use anyhow::{anyhow, bail, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::io::BufRead;

fn distance(
    map: &BTreeMap<(i32, i32), u32>,
    start: (i32, i32),
    end: (i32, i32),
) -> Result<usize> {
    let mut todo = BTreeSet::new();
    let mut seen = BTreeSet::new();
    todo.insert(start);
    for step in 0.. {
        if todo.is_empty() {
            bail!("Ran out of steps");
        }
        let mut next = BTreeSet::new();
        for &(x, y) in &todo {
            if (x, y) == end {
                return Ok(step);
            }
            if !seen.insert((x, y)) {
                continue; // skip duplicates
            }

            let h = map.get(&(x, y)).unwrap();
            for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                if let Some(n) = map.get(&(x + dx, y + dy)) {
                    if *n <= h + 1 {
                        next.insert((x + dx, y + dy));
                    }
                }
            }
        }
        std::mem::swap(&mut next, &mut todo);
    }
    unreachable!()
}

fn main() -> Result<()> {
    let mut map = BTreeMap::new();
    let mut start = None;
    let mut end = None;
    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            let c = match c {
                'S' if start.is_some() => bail!("Multiple 'S' found"),
                'S' => {
                    start = Some((x as i32, y as i32));
                    'a'
                }
                'E' if end.is_some() => bail!("Multiple 'E' found"),
                'E' => {
                    end = Some((x as i32, y as i32));
                    'z'
                }
                c if c.is_ascii_lowercase() => c,
                _ => bail!("Invalid map character '{c}'"),
            };
            map.insert((x as i32, y as i32), c as u32 - 'a' as u32);
        }
    }

    let start = start.ok_or_else(|| anyhow!("No start found"))?;
    let end = end.ok_or_else(|| anyhow!("No end found"))?;

    println!("Part 1: {}", distance(&map, start, end)?);

    let v = map
        .iter()
        .filter(|k| *k.1 == 0)
        .filter_map(|(xy, _)| distance(&map, *xy, end).ok())
        .min()
        .unwrap();
    println!("Part 2: {v}");

    Ok(())
}
