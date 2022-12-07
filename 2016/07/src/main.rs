use std::collections::HashSet;
use std::io::BufRead;

fn has_tls(s: &str) -> bool {
    s.chars()
        .collect::<Vec<char>>()
        .windows(4)
        .any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
}

fn get_abas(s: &str) -> HashSet<[char; 3]> {
    s.chars()
        .collect::<Vec<char>>()
        .windows(3)
        .filter(|w| w[0] == w[2] && w[0] != w[1])
        .map(|w| [w[0], w[1], w[2]])
        .collect()
}

fn get_babs(s: &str) -> HashSet<[char; 3]> {
    s.chars()
        .collect::<Vec<char>>()
        .windows(3)
        .filter(|w| w[0] == w[2] && w[0] != w[1])
        .map(|w| [w[1], w[0], w[1]])
        .collect()
}

fn main() {
    let input = std::io::stdin()
        .lock()
        .lines()
        .map(|line| {
            line.unwrap()
                .replace('[', "]")
                .split(']')
                .map(|s| s.to_owned())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let count = input
        .iter()
        .filter(|line| {
            line.iter().step_by(2).any(|s| has_tls(&s))
                && line.iter().skip(1).step_by(2).all(|s| !has_tls(&s))
        })
        .count();
    println!("Part 1: {}", count);

    let count = input
        .iter()
        .filter(|line| {
            let abas = line
                .iter()
                .step_by(2)
                .flat_map(|w| get_abas(w))
                .collect::<HashSet<_>>();
            let babs = line
                .iter()
                .skip(1)
                .step_by(2)
                .flat_map(|w| get_babs(w))
                .collect::<HashSet<_>>();
            abas.intersection(&babs).count() > 0
        })
        .count();
    println!("Part 2: {}", count);
}
