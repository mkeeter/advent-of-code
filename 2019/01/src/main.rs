use std::io::BufRead;
use std::iter::successors;
use std::str::FromStr;

fn fuel(mass: i32) -> Option<i32> {
    Some(mass / 3 - 2).filter(|m| *m > 0)
}

fn total_fuel(mass: i32) -> i32 {
    successors(Some(mass), |&m| fuel(m)).skip(1).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let masses = std::io::stdin()
        .lock()
        .lines()
        .map(|line| i32::from_str(&line.unwrap()))
        .map(|res| res.expect("Could not parse int"))
        .collect::<Vec<_>>();

    println!(
        "Part 1: {}",
        masses.iter().filter_map(|&m| fuel(m)).sum::<i32>()
    );
    println!(
        "Part 2: {}",
        masses.iter().map(|&m| total_fuel(m)).sum::<i32>()
    );

    Ok(())
}
