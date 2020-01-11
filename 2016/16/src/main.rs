use std::io::Read;
use itertools::Itertools;

fn step(a: &str) -> String {
    let b = a.chars().rev()
        .map(|c| match c {
            '1' => '0',
            '0' => '1',
            _ => panic!("Unknown char {}", c),
            })
        .collect::<String>();
    a.to_owned() + "0" + &b
}

fn checksum(a: &str) -> String {
    if a.len() % 2 == 1 {
        a.to_owned()
    } else {
        let next = a.chars()
            .tuples()
            .map(|(a, b)| if a == b { '1' } else { '0' })
            .collect::<String>();
        checksum(&next)
    }
}

fn run(input: String, disk_size: usize) -> String {
    checksum(&std::iter::successors(Some(input), |s| Some(step(s)))
        .find(|s| s.len() >= disk_size)
        .unwrap()
        .chars()
        .take(disk_size)
        .collect::<String>())
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let input = input.trim().to_string();

    println!("Part 1: {}", run(input.clone(), 272));
    println!("Part 2: {}", run(input.clone(), 35651584));
}
