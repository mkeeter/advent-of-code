use std::io::BufRead;

fn nice(s: &str) -> bool {
    s.chars()
        .filter(|&c| "aeiou".contains(c)).count() >= 3 &&
    s.chars()
        .zip(s.chars().skip(1))
        .any(|(a, b)| a == b) &&
    !s.chars()
        .zip(s.chars().skip(1))
        .any(|p| p == ('a','b') || p == ('c','d') ||
                 p == ('p','q') || p == ('x','y'))
}

fn main() {
    let words = std::io::stdin().lock()
        .lines()
        .map(|r| r.unwrap())
        .collect::<Vec<String>>();

    println!("Part 1: {}", words.iter().filter(|s| nice(s)).count());
}
