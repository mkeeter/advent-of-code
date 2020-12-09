use std::io::BufRead;
use std::str::FromStr;
use std::cmp::Ordering;

const LOOKBACK: usize = 25;

fn main() {
    let p = std::io::stdin().lock().lines()
        .map(|line| u64::from_str(&line.unwrap()).unwrap())
        .collect::<Vec<u64>>();

    let mut buf = [0; LOOKBACK];
    buf.copy_from_slice(&p[0..LOOKBACK]);

    let mut target = 0;
    for (i,n) in p[LOOKBACK..].iter().enumerate() {
        if buf.iter().enumerate().any(|(j, a)| buf[j..].iter()
                                 .any(|b| a + b == *n)) {
            buf[i % LOOKBACK] = *n;
        } else {
            target = *n;
            println!("Part 1: {}", target);
            break;
        }
    }

    'outer: for i in 0..p.len() {
        let mut csum = p[i];
        for j in (i + 1)..p.len() {
            csum += p[j];
            match csum.cmp(&target) {
                Ordering::Equal => {
                    let min = p[i..=j].iter().min().unwrap();
                    let max = p[i..=j].iter().max().unwrap();
                    println!("Part 2: {}", min + max);
                    break 'outer;
                }
                Ordering::Greater => continue 'outer,
                Ordering::Less => (),
            }
        }
    }
}
