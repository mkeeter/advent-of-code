use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use util::get_integers;

const MASK: u32 = 0xFFFFFF;

fn step(mut v: u32) -> u32 {
    v = ((v << 6) ^ v) & MASK;
    v = ((v >> 5) ^ v) & MASK;
    ((v << 11) ^ v) & MASK
}

fn run(seed: u32, cache: &mut HashMap<u32, (u64, HashSet<u32>)>) -> u32 {
    let mut v = seed;
    let mut key = 0u32; // [4 x i8]
    let mut prev_price = (v % 10) as i8;
    for r in 0..2000 {
        v = step(v);
        let price = (v % 10) as i8;
        let delta = price - prev_price;
        key = (key >> 8) | (delta as u8 as u32) << 24;
        if r >= 3 {
            let e = cache.entry(key).or_default();
            if e.1.insert(seed) {
                e.0 += price as u8 as u64;
            }
        }
        prev_price = price;
    }
    v
}

pub fn solve(s: &str) -> (u64, u64) {
    let seeds = get_integers::<u32>(s).collect::<Vec<_>>();
    let (prices, sum) = seeds
        .par_iter()
        .fold(
            || (HashMap::new(), 0),
            |(mut prices, sum), i| {
                let v = u64::from(run(*i, &mut prices));
                (prices, sum + v)
            },
        )
        .reduce(
            || (HashMap::new(), 0),
            |(mut prices, sum), (p, s)| {
                for (k, v) in p {
                    // We just care about merging the sums; we can ignore the
                    // secondary hashset of which values have been seen
                    prices.entry(k).or_default().0 += v.0;
                }
                (prices, sum + s)
            },
        );

    let best_price = prices.values().map(|k| k.0).max().unwrap();

    (sum, best_price)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const EXAMPLE_1: &str = indoc::indoc! {"
            1
            10
            100
            2024
        "};
        assert_eq!(solve(EXAMPLE_1).0, 37327623);

        const EXAMPLE_2: &str = indoc::indoc! {"
            1
            2
            3
            2024
        "};
        assert_eq!(solve(EXAMPLE_2).1, 23);
    }
}
