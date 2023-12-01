use anyhow::Result;
use std::io::BufRead;

fn score<I: Iterator<Item = u32>>(digits: I) -> u32 {
    let mut first = None;
    let mut last = None;
    for i in digits {
        if first.is_none() {
            first = Some(i);
        }
        last = Some(i);
    }
    first.unwrap() * 10 + last.unwrap()
}

fn translate(line: &str) -> impl Iterator<Item = u32> + '_ {
    const DIGITS: [&str; 9] = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    (0..line.len()).filter_map(|i| {
        let substring = line.get(i..).unwrap();
        if let Some(d) = substring.chars().next().unwrap().to_digit(10) {
            Some(d)
        } else {
            for (i, d) in DIGITS.iter().enumerate() {
                if substring.starts_with(d) {
                    return Some(i as u32 + 1);
                }
            }
            None
        }
    })
}

fn main() -> Result<()> {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<String>, _>>()?;

    let sum = lines
        .iter()
        .map(|s| score(s.chars().filter_map(|c| c.to_digit(10))))
        .sum::<u32>();
    println!("Part 1: {sum}");

    let sum = lines.iter().map(|s| score(translate(s))).sum::<u32>();
    println!("Part 2: {sum}");

    // 55330 is too low
    Ok(())
}
