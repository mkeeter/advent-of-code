use std::collections::{HashMap, HashSet};

fn part1() {
    let f = include_str!("../input");

    let mut has2 = 0;
    let mut has3 = 0;
    for line in f.lines() {
        let mut chars = HashMap::new();
        for c in line.chars() {
            let mut value = 1;
            if let Some(x) = chars.get_mut(&c) {
                value = *x + 1;
            }
            chars.insert(c, value);
        }
        let values = chars.values().collect::<HashSet<&i32>>();
        if values.contains(&2) { has2 += 1; }
        if values.contains(&3) { has3 += 1; }
    }
    println!("Has 2: {}, Has 3: {}, mult: {}", has2, has3, has2 * has3);
}

fn part2() {
    let f = include_str!("../input");

    for a in f.lines() {
        for b in f.lines() {
            if a.chars().zip(b.chars()).filter(|(a, b)| a != b).count() == 1 {
                println!("{}", a.chars().zip(b.chars())
                                .filter(|(a, b)| a == b)
                                .map(|(a, _)| a)
                                .collect::<String>());
            }
        }
    }
}

fn main() {
    part1();
    part2();
}
