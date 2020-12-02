use std::io::BufRead;
use std::str::FromStr;

use regex::Regex;

fn main() {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]+)$").unwrap();

    let out = std::io::stdin().lock().lines().map(
        |line| {
            let line = line.unwrap();
            let caps = re.captures(&line).unwrap();
            let a = usize::from_str(caps.get(1).unwrap().as_str()).unwrap();
            let b = usize::from_str(caps.get(2).unwrap().as_str()).unwrap();
            let chr = caps.get(3).unwrap().as_str().chars().next().unwrap();
            let pwd = caps.get(4).unwrap().as_str();

            let n = pwd.chars().filter(|&c| c == chr).count();
            let p1 = n >= a && n <= b;

            let p2 = (((pwd.chars().nth(a - 1).unwrap() == chr) as u8) +
                      ((pwd.chars().nth(b - 1).unwrap() == chr) as u8)) == 1;

            (p1 as usize, p2 as usize)
        }
    ).fold((0,0), |(a, b), (c, d)| (a + c, b + d));

    println!("Part 1: {}", out.0);
    println!("Part 2: {}", out.1);
}
