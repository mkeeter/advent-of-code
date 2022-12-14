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

fn run(map: &BTreeMap<Pos, Tile>, floor: bool) -> usize {
    let y_max = map.keys().map(|p| p.y).max().unwrap_or(0);
    let mut map = map.clone();
    const SOURCE: Pos = Pos { x: 500, y: 0 };
    'outer: while !map.contains_key(&SOURCE) {
        let mut pos = Pos { x: 500, y: 0 };
        loop {
            let y = pos.y + 1;
            let below = Pos { x: pos.x, y };
            if below.y == y_max + 2 {
                if floor {
                    map.insert(pos, Tile::Sand);
                    break;
                } else {
                    break 'outer;
                }
            } else if !map.contains_key(&below) {
                pos = below;
            } else {
                let left = Pos { x: pos.x - 1, y };
                if !map.contains_key(&left) {
                    pos = left;
                } else {
                    let right = Pos { x: pos.x + 1, y };
                    if !map.contains_key(&right) {
                        pos = right;
                    } else {
                        map.insert(pos, Tile::Sand);
                        break;
                    }
                }
            }
        }
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
