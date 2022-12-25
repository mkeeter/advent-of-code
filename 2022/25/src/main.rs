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
    let mut remainder = 0;
    while i != 0 {
        let (c, new_remainder) = match (i % 5) + remainder {
            0 => (b'0', 0),
            1 => (b'1', 0),
            2 => (b'2', 0),
            3 => (b'=', 1),
            4 => (b'-', 1),
            5 => (b'0', 1),
            _ => unreachable!(),
        };
        remainder = new_remainder;
        out.push(c);
        i /= 5;
    }
    if remainder != 0 {
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
