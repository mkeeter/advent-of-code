use std::io::{BufReader, BufRead};
use std::fs::File;
use std::collections::HashSet;

fn parse_line(line: &String) -> i32 {
    let mut i = 0;
    let mut sign = 1;
    for c in line.chars() {
        if c == '+' {
            sign = 1;
        } else if c == '-' {
            sign = -1;
        } else {
            i = i * 10 + (c.to_digit(10).unwrap() as i32);
        }
    }
    return i * sign;
}

fn main() {
    let f = File::open("input").expect("file not found");
    let f = BufReader::new(f);

    let mut deltas = Vec::new();
    for line in f.lines() {
        deltas.push(parse_line(&line.unwrap()));
    }

    let sum: i32 = deltas.iter().sum();
    println!("wavelength: {}", sum);

    let mut seen = HashSet::new();
    let mut wavelength = 0;
    loop {
        for i in deltas.iter() {
            wavelength += i;
            if seen.contains(&wavelength) {
                println!("duplicate: {}", wavelength);
                return;
            } else {
                seen.insert(wavelength);
            }
        }
    }
}
