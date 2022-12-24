use anyhow::{anyhow, bail, Result};
use std::{collections::BTreeSet, io::BufRead};

struct Blizzard {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
}

impl Blizzard {
    fn new(x: i64, y: i64, dx: i64, dy: i64) -> Self {
        Self { x, y, dx, dy }
    }
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
enum Direction {
    Forward,
    Backward,
    ForwardWithSnacks,
}

fn main() -> Result<()> {
    let mut blizzards = vec![];
    let mut walls = BTreeSet::new();
    let mut start = None;
    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            let x = x as i64;
            let y = y as i64;
            match c {
                '.' if y == 0 => start = Some((x, y)),
                '.' => (),
                '#' => _ = walls.insert((x, y)),
                '>' => blizzards.push(Blizzard::new(x, y, 1, 0)),
                '<' => blizzards.push(Blizzard::new(x, y, -1, 0)),
                '^' => blizzards.push(Blizzard::new(x, y, 0, -1)),
                'v' => blizzards.push(Blizzard::new(x, y, 0, 1)),
                c => bail!("Invalid input character '{c}'"),
            }
        }
    }
    let start = start.ok_or_else(|| anyhow!("No start position found"))?;

    let width = walls.iter().map(|p| p.0).max().unwrap_or(0);
    let height = walls.iter().map(|p| p.1).max().unwrap_or(0);

    const DIRS: [(i64, i64); 5] = [(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)];

    let mut todo = BTreeSet::new();
    todo.insert((start, Direction::Forward));
    let mut part1_done = false;
    for round in 0.. {
        if !part1_done && todo.iter().any(|((_x, y), _d)| *y == height) {
            println!("Part 1: {round}");
            part1_done = true;
        }
        if todo.iter().any(|((_x, y), d)| {
            *d == Direction::ForwardWithSnacks && *y == height
        }) {
            println!("Part 2: {round}");
            break;
        }

        let wrap = |p, size| {
            if p == 0 {
                size - 1
            } else if p == size {
                1
            } else {
                p
            }
        };
        let mut next = BTreeSet::new();
        for b in &mut blizzards {
            b.x = wrap(b.x + b.dx, width);
            b.y = wrap(b.y + b.dy, height);
        }
        let mut blocked = walls.clone();
        blocked.extend(blizzards.iter().map(|b| (b.x, b.y)));
        for ((x, y), d) in todo.into_iter() {
            let next_direction = match d {
                Direction::Forward if y == height => Direction::Backward,
                Direction::Backward if y == 0 => Direction::ForwardWithSnacks,
                _ => d,
            };
            for (dx, dy) in DIRS {
                let x = x + dx;
                let y = y + dy;
                if y >= 0 && y <= height && !blocked.contains(&(x, y)) {
                    next.insert(((x, y), next_direction));
                }
            }
        }
        todo = next;
    }

    Ok(())
}
