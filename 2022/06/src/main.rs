use anyhow::{bail, Result};
use std::io::Read;

fn run(masks: &[u32], size: usize) -> Result<usize> {
    let mut accum = 0u32;
    for (i, mask) in masks.iter().enumerate() {
        accum ^= mask;
        if i >= size {
            accum ^= masks[i - size];
            if accum.count_ones() as usize == size {
                return Ok(i + 1);
            }
        }
    }
    bail!("Marker not found")
}

fn main() -> Result<()> {
    let mut input = vec![];
    std::io::stdin().read_to_end(&mut input)?;

    let masks: Vec<u32> = input
        .into_iter()
        .filter(|c| (*c as char).is_ascii_lowercase())
        .map(|c| 1u32 << (c as u32 - 'a' as u32))
        .collect();

    println!("Part 1: {}", run(&masks, 4)?);
    println!("Part 2: {}", run(&masks, 14)?);
    Ok(())
}
