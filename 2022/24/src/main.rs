use anyhow::{anyhow, bail, Result};
use std::{collections::BTreeSet, io::BufRead};

struct Blizzard {
    x: i64,
    y: i64,
    dx: i64,
    dy: i64,
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
                '.' => {
                    if y == 0 {
                        start = Some((x, y))
                    }
                }
                '#' => {
                    walls.insert((x, y));
                }
                '>' => blizzards.push(Blizzard { x, y, dx: 1, dy: 0 }),
                '<' => blizzards.push(Blizzard {
                    x,
                    y,
                    dx: -1,
                    dy: 0,
                }),
                '^' => blizzards.push(Blizzard {
                    x,
                    y,
                    dx: 0,
                    dy: -1,
                }),
                'v' => blizzards.push(Blizzard { x, y, dx: 0, dy: 1 }),
                c => bail!("Invalid input character '{c}'"),
            }
        }
    }
    let start = start.ok_or_else(|| anyhow!("No start position found"))?;

    let width = walls.iter().map(|p| p.0).max().unwrap_or(0);
    let height = walls.iter().map(|p| p.1).max().unwrap_or(0);
    println!("Got map {width} {height} {start:?}");

    const DIRS: [(i64, i64); 5] = [(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)];

    let mut todo = BTreeSet::new();
    todo.insert(start);
    for round in 0.. {
        if todo.iter().any(|(_x, y)| *y == height) {
            println!("Escaped at round {round}");
            break;
        }

        let mut next = BTreeSet::new();
        for b in &mut blizzards {
            b.x += b.dx;
            b.y += b.dy;
            if b.x == 0 {
                b.x = width - 1;
            } else if b.x == width {
                b.x = 1;
            }
            if b.y == 0 {
                b.y = height - 1;
            } else if b.y == height {
                b.y = 1;
            }
        }
        let mut blocked = walls.clone();
        blocked.extend(blizzards.iter().map(|b| (b.x, b.y)));
        for (x, y) in todo.into_iter() {
            for (dx, dy) in DIRS {
                let x = x + dx;
                let y = y + dy;
                if y >= 0 && !blocked.contains(&(x, y)) {
                    next.insert((x, y));
                }
            }
        }
        todo = next;
    }

    println!("Hello, world!");
    Ok(())
}
