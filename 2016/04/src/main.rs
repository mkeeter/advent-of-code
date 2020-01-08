use std::collections::HashMap;
use std::str::FromStr;
use std::io::BufRead;
use regex::Regex;

fn check(room: &str, checksum: &str) -> bool {
    let mut count = HashMap::new();
    for c in room.chars() {
        if c != '-' {
            *count.entry(c).or_insert(0) += 1;
        }
    }
    let mut sorted = count.into_iter()
        .collect::<Vec<(char, i32)>>();
    sorted.sort_by_key(|(c, n)| (-*n, (*c as i32)));

    checksum == sorted.iter()
        .take(5)
        .map(|(c, _n)| c)
        .collect::<String>()
}

fn decrypt(room: &str, sector: u32) -> String {
    let offset = 'a' as u32;
    let modulo = 'z' as u32 - offset + 1;
    room.chars()
        .map(|c| {
            if c == '-' {
                ' '
            } else {
                (((c as u32 - offset + sector) % modulo) + offset) as u8 as char
            }
        })
        .collect::<String>()
}

fn main() {
    let r = Regex::new(r"(.*)-(\d+)\[(.*)\]").unwrap();
    let input = std::io::stdin().lock().lines()
        .map(|line| {
            let line = line.unwrap();
            let c = r.captures(&line).unwrap();
            let name = c.get(1).unwrap().as_str().to_owned();
            let sector = u32::from_str(c.get(2).unwrap().as_str()).unwrap();
            let checksum = c.get(3).unwrap().as_str().to_owned();
            (name, sector, checksum)
        })
        .collect::<Vec<(String, u32, String)>>();

    let a = input.iter()
        .filter(|line| check(&line.0, &line.2))
        .map(|line| line.1)
        .sum::<u32>();
    println!("Part 1: {}", a);
    for line in input {
        let d = decrypt(&line.0, line.1);
        if d.contains("northpole") {
            println!("Part 2: {} ({})", line.1, d);
        }
    }
}
