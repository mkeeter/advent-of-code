use anyhow::Result;
use std::collections::BTreeSet;
use std::io::BufRead;

fn look<I: Clone + Iterator<Item = i32>>(
    iter: I,
) -> impl Iterator<Item = usize> {
    let mut highest = -1;
    iter.enumerate().filter_map(move |(i, t)| {
        if t > highest {
            highest = t;
            Some(i)
        } else {
            None
        }
    })
}

fn bilook(row: &[i32]) -> BTreeSet<usize> {
    look(row.iter().cloned())
        .chain(look(row.iter().cloned().rev()).map(|i| row.len() - i - 1))
        .collect()
}

fn main() -> Result<()> {
    let lines = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| (c as u32 - '0' as u32) as i32)
                .collect()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut out = BTreeSet::new();
    for (i, row) in lines.iter().enumerate() {
        out.extend(bilook(row).into_iter().map(|c| (i, c)));
    }
    for j in 0..lines[0].len() {
        let col: Vec<i32> = lines.iter().map(|row: &Vec<i32>| row[j]).collect();
        out.extend(bilook(&col).into_iter().map(|r| (r, j)));
    }

    println!("Part 1: {}", out.len());
    Ok(())
}
