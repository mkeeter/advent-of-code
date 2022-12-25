use anyhow::{bail, Result};
use std::io::BufRead;

fn decode(s: &str) -> Result<i64> {
    let mut out = 0;
    for c in s.chars() {
        out *= 5;
        out += match c {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => bail!("Invalid character '{c}'"),
        }
    }
    Ok(out)
}

fn encode(mut i: i64) -> String {
    let mut out = vec![];
    let mut remainder = false;
    while i != 0 {
        let v = (i % 5) + remainder as i64;
        out.push([b'0', b'1', b'2', b'=', b'-'][(v % 5) as usize]);
        remainder = v >= 3;
        i /= 5;
    }
    if remainder {
        out.push(b'1');
    } else if out.is_empty() {
        out.push(b'0');
    }
    out.reverse();
    String::from_utf8(out).unwrap()
}

fn main() -> Result<()> {
    let sum = std::io::stdin()
        .lock()
        .lines()
        .map(|line| decode(&line.unwrap()))
        .sum::<Result<i64>>()?;

    println!("Part 1: {}", encode(sum));
    println!("Part 2: ⭐️");
    Ok(())
}
