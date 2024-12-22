use std::collections::HashMap;
use util::get_integers;

const MASK: u32 = 0xFFFFFF;

fn step(mut v: u32) -> u32 {
    v = ((v << 6) ^ v) & MASK;
    v = ((v >> 5) ^ v) & MASK;
    ((v << 11) ^ v) & MASK
}

fn run(seed: u32, cache: &mut HashMap<u32, HashMap<u32, u8>>) -> u32 {
    let mut v = seed;
    let mut key = 0u32; // [4 x i8]
    let mut prev_price = (v % 10) as i8;
    for r in 0..2000 {
        v = step(v);
        let price = (v % 10) as i8;
        let delta = price - prev_price;
        key = (key >> 8) | (delta as u8 as u32) << 24;
        if r >= 3 {
            cache
                .entry(key)
                .or_default()
                .entry(seed)
                .or_insert(price as u8);
        }
        prev_price = price;
    }
    v
}

pub fn solve(s: &str) -> (u64, u64) {
    let mut sum = 0;
    let seeds = get_integers::<u32>(s).collect::<Vec<_>>();
    let mut prices = HashMap::new();
    for i in seeds.iter().cloned() {
        sum += u64::from(run(i, &mut prices));
    }

    let best_price = prices
        .values()
        .map(|k| k.values().map(|v| u64::from(*v)).sum())
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
