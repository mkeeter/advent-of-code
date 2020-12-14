use std::collections::HashMap;
use std::io::BufRead;
use regex::Regex;

fn unpack(mask: &[(usize, char)], addr: u64, out: &mut Vec<u64>)
{
    match mask.get(0) {
        None => out.push(addr),
        Some((i, c)) => match c {
            '1' => unpack(&mask[1..], addr | (1 << (35 - i)), out),
            '0' => unpack(&mask[1..], addr, out),
            'X' => {
                unpack(&mask[1..], addr | (1 << (35 - i)), out);
                unpack(&mask[1..], addr & (!(1 << (35 - i))), out);
            },
            _ => panic!("Invalid mask char: {}", c),
        }
    }
}

fn main() {
    let stdin = std::io::stdin();
    let lines: Vec<String> = stdin.lock().lines().map(|s| s.unwrap()).collect();

    let imem = Regex::new(r#"^mem\[([0-9]+)\] = ([0-9]+)$"#).unwrap();
    let imask = Regex::new(r#"^mask = ([01X]+)$"#).unwrap();

    let mut mem = HashMap::new();
    let mut mask_set: u64 = 0;
    let mut mask_clear: u64 = 0;

    for line in lines.iter() {
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

    let mut mem = HashMap::new();
    let mut mask = "";
    for line in lines.iter() {
        if let Some(c) = imask.captures(&line) {
            mask = c.get(1).unwrap().as_str();
            assert!(mask.len() == 36);
        } else if let Some(c) = imem.captures(&line) {
            let addr: u64 = c.get(1).unwrap().as_str().parse().unwrap();
            let val: u64 = c.get(2).unwrap().as_str().parse().unwrap();
            let mut out = Vec::new();
            let mask: Vec<_> = mask.chars().enumerate().collect();
            unpack(&mask, addr, &mut out);
            for addr in out {
                mem.insert(addr, val);
            }
        }
    }
    let sum: u64 = mem.iter().map(|(_k, v)| *v).sum();
    println!("Part 2: {}", sum);
}
