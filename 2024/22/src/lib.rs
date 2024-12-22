use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use util::get_integers;

const MASK: u32 = 0xFFFFFF;

fn step(mut v: u32) -> u32 {
    v = ((v << 6) ^ v) & MASK;
    v = ((v >> 5) ^ v) & MASK;
    ((v << 11) ^ v) & MASK
}

fn find_prices(mut v: u32) -> (HashMap<u32, u8>, u32) {
    let mut key = 0u32; // [4 x i8]
    let mut prev_price = (v % 10) as i8;
    let mut out = HashMap::new();
    for r in 0..2000 {
        v = step(v);
        let price = (v % 10) as i8;
        let d = price - prev_price;
        key = (key >> 8) | (d as u8 as u32) << 24;
        if r >= 3 {
            out.entry(key).or_insert(price as u8);
        }
        prev_price = price;
    }
    (out, v)
}

pub fn solve(s: &str) -> (u64, u64) {
    let mut sum = 0;
    let seeds = get_integers::<u32>(s).collect::<Vec<_>>();
    let mut prices = HashMap::new();
    for i in seeds.iter().cloned() {
        let (p, v) = find_prices(i);
        prices.insert(i, p);
        sum += u64::from(v);
    }

    let mut all_seq: HashSet<u32> = HashSet::new();
    for p in prices.values() {
        all_seq.extend(p.keys());
    }

    let best_price = all_seq
        .into_par_iter()
        .map(|seq| {
            prices
                .values()
                .flat_map(|v| v.get(&seq))
                .map(|v| *v as u64)
                .sum::<u64>()
        })
        .max()
        .unwrap();

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

        println!("example 2");
        const EXAMPLE_2: &str = indoc::indoc! {"
            1
            2
            3
            2024
        "};
        assert_eq!(solve(EXAMPLE_2).1, 23);
    }
}
