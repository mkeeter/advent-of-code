use anyhow::{anyhow, bail, Result};
use parse_display::{Display, FromStr};
use std::io::BufRead;

#[derive(Copy, Clone, Debug, FromStr, Display)]
#[display("move {count} from {src} to {dst}")]
struct Move {
    count: usize,
    src: usize,
    dst: usize,
}

fn run(
    mut stacks: Vec<Vec<char>>,
    moves: &[Move],
    advanced: bool,
) -> Result<String> {
    for m in moves {
        for _ in 0..m.count {
            let c = stacks[m.src]
                .pop()
                .ok_or_else(|| anyhow!("Popped empty stack"))?;
            stacks[m.dst].push(c);
        }
        if advanced {
            let n = stacks[m.dst].len() - m.count;
            stacks[m.dst][n..].reverse();
        }
    }
    stacks[1..]
        .iter()
        .map(|s| s.last().ok_or_else(|| anyhow!("Empty stack")))
        .collect()
}

fn main() -> Result<()> {
    let mut stacks: Vec<Vec<char>> = vec![];
    let mut moves = vec![];
    for line in std::io::stdin().lock().lines() {
        let line = line.unwrap();
        if line.trim().starts_with('[') {
            for (i, c) in (1..).zip(line.chars().skip(1).step_by(4)) {
                if i >= stacks.len() {
                    stacks.resize_with(i + 1, Default::default);
                }
                match c {
                    c if c.is_ascii_uppercase() => stacks[i].push(c),
                    ' ' => (),
                    _ => bail!("Invalid box name: '{c}'"),
                }
            }
        } else if let Ok(m) = line.parse::<Move>() {
            moves.push(m);
        }
    }
    for stack in &mut stacks {
        stack.reverse();
    }

    println!("Part 1: {}", run(stacks.clone(), &moves, false)?);
    println!("Part 2: {}", run(stacks, &moves, true)?);

    Ok(())
}
