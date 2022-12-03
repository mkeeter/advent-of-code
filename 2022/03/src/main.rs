use anyhow::{anyhow, bail, Result};
use std::collections::BTreeSet;
use std::io::BufRead;

fn score(c: char) -> Result<usize> {
    match c {
        'a'..='z' => Ok((c as u32 - 'a' as u32) as usize + 1),
        'A'..='Z' => Ok((c as u32 - 'A' as u32) as usize + 27),
        _ => bail!("Invalid character '{c}'"),
    }
}

fn find_common_char(line: &[char]) -> Result<usize> {
    let (a, b) = line.split_at(line.len() / 2);
    let a: BTreeSet<char> = a.iter().cloned().collect();
    let c = *b
        .iter()
        .find(|c| a.contains(c))
        .ok_or_else(|| anyhow!("No common character"))?;
    score(c)
}

fn main() -> Result<()> {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect::<Vec<Vec<char>>>();

    let out = lines
        .iter()
        .map(|line| find_common_char(line))
        .sum::<Result<usize, _>>()?;
    println!("Part 1: {out}");

    let out = lines
        .chunks(3)
        .map(|lines| {
            let common = lines
                .iter()
                .cloned()
                .reduce(|a, b| {
                    let a: BTreeSet<char> = a.iter().cloned().collect();
                    let b: BTreeSet<char> = b.iter().cloned().collect();
                    a.intersection(&b).cloned().collect()
                })
                .ok_or_else(|| anyhow!("Fold failed; empty input?"))?;
            if common.len() != 1 {
                bail!("Too many common items: {common:?}");
            }
            score(common[0])
        })
        .sum::<Result<usize, _>>()?;
    println!("Part 2: {out}");

    Ok(())
}
