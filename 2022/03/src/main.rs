use anyhow::{anyhow, bail, Result};
use std::io::BufRead;

fn score(c: char) -> Result<u64> {
    match c {
        'a'..='z' => Ok((c as u32 - 'a' as u32) as u64 + 1),
        'A'..='Z' => Ok((c as u32 - 'A' as u32) as u64 + 27),
        _ => Err(anyhow!("Invalid character '{c}'")),
    }
}

fn group(s: &str) -> Result<u64> {
    s.chars().map(score).try_fold(0, |a, b| Ok(a | (1 << b?)))
}

fn run(iter: &[u64], group_size: usize) -> Result<usize> {
    iter.chunks(group_size)
        .map(|group| {
            let common = group
                .iter()
                .cloned()
                .reduce(|a, b| a & b)
                .ok_or_else(|| anyhow!("Fold failed; empty input?"))?;
            if common.count_ones() != 1 {
                bail!("Too many common items: {common:b}");
            }
            Ok(common.trailing_zeros() as usize)
        })
        .sum()
}

fn main() -> Result<()> {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>();

    // Convert to a bitfield, where trailing_zeros() is the character score
    let compartments: Vec<u64> = lines
        .iter()
        .flat_map(|line| {
            // For part 1, we need to split each line into two compartments
            let (a, b) = line.split_at(line.len() / 2);
            [a, b].into_iter()
        })
        .map(group)
        .collect::<Result<_>>()?;

    let out = run(&compartments, 2)?;
    println!("Part 1: {out}");

    let elves: Vec<u64> =
        lines.iter().map(|s| group(s)).collect::<Result<_>>()?;
    let out = run(&elves, 3)?;
    println!("Part 2: {out}");

    Ok(())
}
