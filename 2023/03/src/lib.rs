use std::collections::{BTreeMap, BTreeSet};

use arrayvec::ArrayVec;

fn part1(numbers: &[((i64, i64), u32)], symbols: &[((i64, i64), char)]) -> u32 {
    // Find the 3x3 neighborhood around each symbol
    let mut ns = BTreeSet::new();
    for ((x, y), _) in symbols {
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                ns.insert((x + dx, y + dy));
            }
        }
    }
    // Find any numbers which overlap with those neighborhoods
    let mut out = 0;
    for ((x, y), v) in numbers {
        if (0..=v.ilog10()).any(|dx| ns.contains(&(*x + dx as i64, *y))) {
            out += v;
        }
    }
    out
}

fn part2(numbers: &[((i64, i64), u32)], gears: &[(i64, i64)]) -> u32 {
    // Build a map from (x, y) -> index of number in `nums`
    let mut numspan = BTreeMap::new();
    let mut nums = vec![];
    for (i, ((x, y), v)) in numbers.iter().enumerate() {
        for dx in 0..=v.ilog10() as i64 {
            numspan.insert((*x + dx, *y), i);
        }
        nums.push(*v);
    }

    let mut out = 0;
    for (x, y) in gears {
        // Find numbers (by index) which are neighbors of this gear
        let mut ns: ArrayVec<usize, 9> = ArrayVec::new();
        for dx in [-1, 0, 1] {
            for dy in [-1, 0, 1] {
                if let Some(i) = numspan.get(&(x + dx, y + dy)) {
                    ns.push(*i);
                }
            }
        }
        // Find the product of unique numbers in the list
        ns.sort();
        let mut prev = None;
        let mut count = 0;
        let mut prod = 1;
        for i in &ns {
            if prev.map(|p| p != i).unwrap_or(true) {
                count += 1;
                prod *= nums[*i];
            }
            prev = Some(i);
        }
        if count == 2 {
            out += prod;
        }
    }
    out
}

pub fn solve(s: &str) -> (String, String) {
    let lines = s.lines().collect::<Vec<_>>();

    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    let mut gears = Vec::new();
    for (y, line) in lines.iter().enumerate() {
        let mut number = None;
        for (x, c) in line.chars().enumerate() {
            if let Some(d) = c.to_digit(10) {
                let (pos, prev) = number.unwrap_or(((x as i64, y as i64), 0));
                number = Some((pos, prev * 10 + d));
            } else {
                if c != '.' {
                    symbols.push(((x as i64, y as i64), c));
                }
                if c == '*' {
                    gears.push((x as i64, y as i64));
                }
                numbers.extend(number.take());
            }
        }
        numbers.extend(number.take());
    }

    let out = std::thread::scope(|s| {
        let p1 = s.spawn(|| part1(&numbers, &symbols).to_string());
        let p2 = s.spawn(|| part2(&numbers, &gears).to_string());
        (p1.join().unwrap(), p2.join().unwrap())
    });
    out
}
