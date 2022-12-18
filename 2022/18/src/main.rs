use anyhow::Result;
use parse_display::{Display, FromStr};
use std::{collections::BTreeSet, io::BufRead};

#[derive(Copy, Clone, Display, FromStr, Ord, PartialOrd, Eq, PartialEq)]
#[display("{x},{y},{z}")]
struct Pos {
    x: i64,
    y: i64,
    z: i64,
}

impl Pos {
    fn neighbors(&self) -> impl Iterator<Item = Pos> + '_ {
        [-1, 1].into_iter().flat_map(move |sign| {
            [(0, 0, 1), (0, 1, 0), (1, 0, 0)].into_iter().map(
                move |(dx, dy, dz)| Self {
                    x: self.x + sign * dx,
                    y: self.y + sign * dy,
                    z: self.z + sign * dz,
                },
            )
        })
    }
}

fn run<F: Fn(&Pos) -> bool>(rocks: &BTreeSet<Pos>, pred: F) -> usize {
    rocks
        .iter()
        .flat_map(|r| r.neighbors())
        .filter(|n| pred(n))
        .count()
}

fn main() -> Result<()> {
    let rocks = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse())
        .collect::<Result<BTreeSet<Pos>, _>>()?;

    println!("Part 1: {}", run(&rocks, |r| !rocks.contains(r)));

    // Flood fill to find out what's accessible to the open air
    let xmin = rocks.iter().map(|r| r.x).min().unwrap_or(0) - 1;
    let xmax = rocks.iter().map(|r| r.x).max().unwrap_or(0) + 1;
    let ymin = rocks.iter().map(|r| r.y).min().unwrap_or(0) - 1;
    let ymax = rocks.iter().map(|r| r.y).max().unwrap_or(0) + 1;
    let zmin = rocks.iter().map(|r| r.z).min().unwrap_or(0) - 1;
    let zmax = rocks.iter().map(|r| r.z).max().unwrap_or(0) + 1;
    let mut air = BTreeSet::new();
    let mut todo = vec![Pos {
        x: xmin,
        y: ymin,
        z: zmin,
    }];
    while let Some(t) = todo.pop() {
        if air.insert(t)
            && (xmin..=xmax).contains(&t.x)
            && (ymin..=ymax).contains(&t.y)
            && (zmin..=zmax).contains(&t.z)
        {
            todo.extend(t.neighbors().filter(|n| !rocks.contains(n)));
        }
    }
    println!("Part 2: {}", run(&rocks, |r| air.contains(r)));
    Ok(())
}
