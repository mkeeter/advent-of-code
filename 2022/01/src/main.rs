use anyhow::Result;
use std::io::BufRead;

fn main() -> Result<()> {
    let nums = std::io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<String>, _>>()?;

    let mut elves = nums
        .split(String::is_empty)
        .map(|lines| lines.iter().map(|s| s.parse::<u64>()).sum())
        .collect::<Result<Vec<_>, _>>()?;
    elves.sort_unstable();

    println!("Part 1: {}", elves.last().unwrap());
    println!("Part 2: {}", elves.iter().rev().take(3).sum::<u64>());

    Ok(())
}
