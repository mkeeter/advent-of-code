use std::collections::HashMap;
use std::io::BufRead;
use regex::Regex;

fn main() {
    let stdin = std::io::stdin();

    let imem = Regex::new(r#"^mem\[([0-9]+)\] = ([0-9]+)$"#).unwrap();
    let imask = Regex::new(r#"^mask = ([01X]+)$"#).unwrap();

    let mut mem = HashMap::new();
    let mut mask_set: u64 = 0;
    let mut mask_clear: u64 = 0;

    for line in stdin.lock().lines() {
        let line = line.unwrap();
        if let Some(c) = imask.captures(&line) {
            mask_set = 0;
            mask_clear = 0;
            let m = c.get(1).unwrap().as_str();
            for (i, c) in m.chars().enumerate() {
                match c {
                    '1' => mask_set |= 1 << (35 - i),
                    '0' => mask_clear |= 1 << (35 - i),
                    'X' => continue,
                    _ => panic!("Invalid mask char: {}", c),
                }
            }
        } else if let Some(c) = imem.captures(&line) {
            let addr: usize = c.get(1).unwrap().as_str().parse().unwrap();
            let val: u64 = c.get(2).unwrap().as_str().parse().unwrap();
            mem.insert(addr, (val | mask_set) & (!mask_clear));
        } else {
            println!("Failed to parse: {}", line);
        }
    }
    let sum: u64 = mem.iter().map(|(_k, v)| *v).sum();
    println!("Part 1: {}", sum);
}
