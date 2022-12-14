use anyhow::Result;
use parse_display::{Display, FromStr};
use std::{collections::BTreeMap, io::BufRead};

#[derive(
    Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug, FromStr, Display,
)]
#[display("{x},{y}")]
struct Pos {
    y: i64,
    x: i64,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Rock,
    Sand,
}

const SOURCE: Pos = Pos { x: 500, y: 0 };
fn drop(map: &BTreeMap<Pos, Tile>, floor: bool, y_max: i64) -> Option<Pos> {
    let mut pos = SOURCE;
    while pos.y != y_max {
        match [(0, 1), (-1, 1), (1, 1)]
            .iter()
            .map(|(dx, dy)| Pos {
                x: pos.x + dx,
                y: pos.y + dy,
            })
            .find(|next| !map.contains_key(next))
        {
            Some(next) => pos = next,
            None => return Some(pos),
        }
    }
    if floor {
        Some(pos)
    } else {
        None
    }
}

fn run(map: &BTreeMap<Pos, Tile>, floor: bool) -> usize {
    let mut map = map.clone();
    let y_max = map.keys().rev().next().map(|p| p.y).unwrap_or(0) + 1;
    while !map.contains_key(&SOURCE) {
        match drop(&map, floor, y_max) {
            Some(v) => map.insert(v, Tile::Sand),
            None => break,
        };
    }
    map.values().filter(|v| **v == Tile::Sand).count()
}

fn main() -> Result<()> {
    let mut map = BTreeMap::new();
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        let positions = line
            .split(" -> ")
            .map(|p| p.parse())
            .collect::<Result<Vec<Pos>, _>>()?;
        for (a, b) in positions.iter().zip(positions.iter().skip(1)) {
            let mut a = *a;
            while a != *b {
                map.insert(a, Tile::Rock);
                a.x += (b.x - a.x).signum();
                a.y += (b.y - a.y).signum();
            }
            map.insert(*b, Tile::Rock);
        }
    }

    println!("Part 1: {}", run(&map, false));
    println!("Part 2: {}", run(&map, true));

    Ok(())
}
