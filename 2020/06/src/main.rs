use std::io::Read;
use std::collections::HashSet;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let lines: Vec<&str> = input.split("\n\n").collect();
    let p1: usize = lines.iter()
        .map(|g| g.chars()
            .filter(|c| c.is_alphabetic())
            .collect::<HashSet<_>>()
            .len())
        .sum();
    println!("Part 1: {}", p1);

    let chars: HashSet<_> = input.chars().collect();
    let p2: usize = lines.iter()
        .map(|g| g.lines()
            .map(|n| n.chars().collect::<HashSet<_>>())
            .fold(chars.clone(), |a, b| a.intersection(&b).copied().collect())
            .len())
        .sum();
    println!("Part 2: {}", p2);
}
