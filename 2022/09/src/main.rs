use anyhow::{anyhow, bail, Result};
use std::collections::BTreeSet;
use std::io::BufRead;

struct Command {
    dir: (i64, i64),
    count: usize,
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
        for _ in 0..cmd.count {
            rope[0].0 += cmd.dir.0;
            rope[0].1 += cmd.dir.1;
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
        .map(|line| -> Result<Command> {
            let line = line.unwrap();
            let mut iter = line.split(' ');
            let dir = match iter
                .next()
                .ok_or_else(|| anyhow!("No direction code"))?
            {
                "U" => (0, 1),
                "D" => (0, -1),
                "L" => (-1, 0),
                "R" => (1, 0),
                c => bail!("Invalid direction '{c}'"),
            };
            let count =
                iter.next().ok_or_else(|| anyhow!("No count"))?.parse()?;
            if let Some(d) = iter.next() {
                bail!("Extra data in line: '{d}'");
            }
            Ok(Command { dir, count })
        })
        .collect::<Result<Vec<Command>, _>>()?;

    println!("Part 1: {}", run(&commands, 2)?);
    println!("Part 1: {}", run(&commands, 10)?);
    Ok(())
}
