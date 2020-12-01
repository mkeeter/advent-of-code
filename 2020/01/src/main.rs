use std::io::BufRead;
use std::str::FromStr;
use std::collections::{HashSet, HashMap};

fn main() -> () {
    let nums = std::io::stdin().lock().lines()
        .map(|line| i32::from_str(&line.unwrap()))
        .map(|res| res.expect("Could not parse int"))
        .collect::<HashSet<_>>();

    for a in nums.iter() {
        if nums.contains(&(2020 - a)) {
            println!("Part 1: {}", a * (2020 - a));
            break;
        }
    }

    let mut sums: HashMap<i32, i32> = HashMap::new();
    for (i, a) in nums.iter().enumerate() {
        for b in nums.iter().skip(i + 1) {
            sums.insert(a + b, a * b);
        }
    }
    for a in nums.iter() {
        if let Some(b) = sums.get(&(2020 - a)) {
            println!("Part 2: {}", a * b);
            break;
        }
    }
}
