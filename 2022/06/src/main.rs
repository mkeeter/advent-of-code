use anyhow::{bail, Result};
use std::io::Read;

fn run(masks: &[u64], size: usize) -> Result<usize> {
    let mut accum = 0u64;
    for (i, mask) in masks.iter().enumerate() {
        accum ^= mask;
        if i >= size {
            accum ^= masks[i - size] as u64;
            if accum.count_ones() as usize == size {
                return Ok(i + 1);
            }
        }
    }
    bail!("List does not contain at least {size} items")
}

fn main() -> Result<()> {
    let mut input = vec![];
    std::io::stdin().read_to_end(&mut input)?;

    let masks: Vec<u64> = input
        .into_iter()
        .filter(|c| (*c as char).is_ascii_lowercase())
        .map(|c| 1u64 << (c as u32 - 'a' as u32))
        .collect();

    println!("Part 1: {}", run(&masks, 4)?);
    println!("Part 2: {}", run(&masks, 14)?);
    Ok(())
}
