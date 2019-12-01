use std::io::{BufReader, BufRead};
use std::fs::File;
use std::str::FromStr;
use std::iter::successors;

fn fuel(mass: &i32) -> Option<i32> {
    Some(*mass / 3 - 2).filter(|m| *m > 0)
}

fn total_fuel(mass: &i32) -> i32 {
    successors(Some(*mass), fuel).skip(1).sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let f = File::open("input").expect("file not found");
    let f = BufReader::new(f);

    let masses = f.lines()
        .map(|line| i32::from_str(&line.unwrap()))
        .map(|res| res.expect("Could not parse int"))
        .collect::<Vec<i32>>();

    println!("Part 1: {}", masses.iter().filter_map(fuel).sum::<i32>());
    println!("Part 2: {}", masses.iter().map(total_fuel).sum::<i32>());

    Ok(())
}
