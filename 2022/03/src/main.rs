use anyhow::{anyhow, bail, Result};
use std::collections::BTreeSet;
use std::io::BufRead;

fn run<'a, I: Iterator<Item = &'a str>>(
    iter: I,
    group_size: usize,
) -> Result<usize> {
    let lines = iter
        .map(|i| i.chars().collect::<BTreeSet<char>>())
        .collect::<Vec<_>>();
    lines
        .chunks(group_size)
        .map(|group| {
            let common = group
                .iter()
                .cloned()
                .reduce(|a, b| a.intersection(&b).cloned().collect())
                .ok_or_else(|| anyhow!("Fold failed; empty input?"))?;
            if common.len() != 1 {
                bail!("Too many common items: {common:?}");
            }
            let c = common.into_iter().next().unwrap();
            match c {
                'a'..='z' => Ok((c as u32 - 'a' as u32) as usize + 1),
                'A'..='Z' => Ok((c as u32 - 'A' as u32) as usize + 27),
                _ => Err(anyhow!("Invalid character '{c}'")),
            }
        })
        .sum()
}

fn main() -> Result<()> {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .collect::<Vec<String>>();

    let compartments = lines.iter().flat_map(|line| {
        let (a, b) = line.split_at(line.len() / 2);
        [a, b].into_iter()
    });
    let out = run(compartments, 2)?;
    println!("Part 1: {out}");

    let out = run(lines.iter().map(String::as_str), 3)?;
    println!("Part 2: {out}");

    Ok(())
}
