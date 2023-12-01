use anyhow::Result;
use std::io::BufRead;

#[derive(Default)]
struct Digits {
    first: Option<u32>,
    last: Option<u32>,
}

impl Digits {
    fn record(&mut self, i: u32) {
        self.first.get_or_insert(i);
        self.last.replace(i);
    }
    fn value(&self) -> u32 {
        self.first.unwrap() * 10 + self.last.unwrap()
    }
}

fn score1(s: &str) -> u32 {
    let mut out = Digits::default();
    for c in s.chars().filter_map(|c| c.to_digit(10)) {
        out.record(c);
    }
    out.value()
}

fn score2(line: &str) -> u32 {
    const DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut cs = line.chars();
    let mut out = Digits::default();
    loop {
        if let Some((i, _)) = DIGITS.iter().enumerate().find(|(_, d)| {
            cs.clone()
                .chain(std::iter::repeat(' '))
                .zip(d.chars())
                .all(|(a, b)| a == b)
        }) {
            out.record(i as u32 + 1);
        }
        if let Some(c) = cs.next() {
            if let Some(d) = c.to_digit(10) {
                out.record(d);
            }
        } else {
            break out.value();
        }
    }
}

fn main() -> Result<()> {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<String>, _>>()?;

    let sum = lines.iter().map(|s| score1(s)).sum::<u32>();
    println!("Part 1: {sum}");

    let sum = lines.iter().map(|s| score2(s)).sum::<u32>();
    println!("Part 2: {sum}");

    Ok(())
}
