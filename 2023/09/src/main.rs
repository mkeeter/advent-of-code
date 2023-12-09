use anyhow::Result;
use std::io::BufRead;

fn main() -> Result<()> {
    let mut lines = std::io::stdin()
        .lock()
        .lines()
        .map(|s| {
            s.unwrap()
                .split_whitespace()
                .map(|p| p.parse::<i64>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<i64>>>();

    let mut p1 = 0;
    let mut p2 = 0;
    let mut heads = vec![];
    let mut tails = vec![];
    for line in lines.iter_mut() {
        heads.clear();
        tails.clear();
        while !line.iter().all(|v| *v == 0) {
            heads.push(line[0]);
            for i in 1..line.len() {
                line[i - 1] = line[i] - line[i - 1];
            }
            tails.push(line.pop().unwrap());
        }
        while tails.len() > 1 {
            let t = tails.pop().unwrap();
            *tails.last_mut().unwrap() += t;
        }
        p1 += tails.pop().unwrap();
        while heads.len() > 1 {
            let h = heads.pop().unwrap();
            *heads.last_mut().unwrap() -= h;
        }
        p2 += heads.pop().unwrap();
    }
    println!("Part 1: {p1}");
    println!("Part 2: {p2}");

    Ok(())
}
