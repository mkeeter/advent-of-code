use anyhow::{bail, Result};
use parse_display::{Display, FromStr};
use std::collections::BTreeSet;
use std::io::BufRead;

#[derive(Copy, Clone, Debug, FromStr, Display)]
#[display("{dir} {count}")]
struct Command {
    dir: char,
    count: usize,
}

impl Command {
    fn dir(&self) -> Result<(i64, i64)> {
        match self.dir {
            'U' => Ok((0, 1)),
            'D' => Ok((0, -1)),
            'L' => Ok((-1, 0)),
            'R' => Ok((1, 0)),
            c => bail!("Invalid direction '{c}'"),
        }
    }
}

fn adjust((hx, hy): (i64, i64), (tx, ty): (i64, i64)) -> (i64, i64) {
    let dx = hx - tx;
    let dy = hy - ty;

    if dx.abs() > 1 || dy.abs() > 1 {
        (tx + dx.signum(), ty + dy.signum())
    } else {
        (tx, ty)
    }
}

fn run(commands: &[Command], length: usize) -> Result<usize> {
    let mut rope = vec![(0i64, 0i64); length];
    let mut seen = BTreeSet::new();
    seen.insert(*rope.last().unwrap());
    for cmd in commands {
        let (dx, dy) = cmd.dir()?;
        for _ in 0..cmd.count {
            rope[0].0 += dx;
            rope[0].1 += dy;
            for i in 1..length {
                rope[i] = adjust(rope[i - 1], rope[i]);
            }
            seen.insert(*rope.last().unwrap());
        }
    }
    Ok(seen.len())
}

fn main() -> Result<()> {
    let commands = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse())
        .collect::<Result<Vec<Command>, _>>()?;

    println!("Part 1: {}", run(&commands, 2)?);
    println!("Part 1: {}", run(&commands, 10)?);
    Ok(())
}
