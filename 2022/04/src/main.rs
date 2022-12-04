use anyhow::Result;
use parse_display::{Display, FromStr};
use std::io::BufRead;

#[derive(Copy, Clone, Debug, FromStr, Display)]
#[display("{lo}-{hi}")]
struct Section {
    lo: i64,
    hi: i64,
}

impl Section {
    fn contains(&self, other: &Section) -> bool {
        self.lo <= other.lo && self.hi >= other.hi
    }
    fn intersects(&self, other: &Section) -> bool {
        (other.lo >= self.lo && other.lo <= self.hi)
            || (self.lo >= other.lo && self.lo <= other.hi)
    }
}

#[derive(Copy, Clone, Debug, FromStr, Display)]
#[display("{a},{b}")]
struct Input {
    a: Section,
    b: Section,
}

fn main() -> Result<()> {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse::<Input>())
        .collect::<Result<Vec<Input>, _>>()?;

    let contained = lines
        .iter()
        .filter(|i| i.a.contains(&i.b) || i.b.contains(&i.a))
        .count();
    println!("Part 1: {contained}");

    let overlaps = lines.iter().filter(|i| i.a.intersects(&i.b)).count();
    println!("Part 2: {overlaps}");
    Ok(())
}
