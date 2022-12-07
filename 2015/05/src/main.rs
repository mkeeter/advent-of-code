use itertools::Itertools;
use std::collections::HashMap;
use std::io::BufRead;

fn nice1(s: &str) -> bool {
    s.chars().filter(|&c| "aeiou".contains(c)).count() >= 3
        && s.chars().zip(s.chars().skip(1)).any(|(a, b)| a == b)
        && !s
            .chars()
            .tuple_windows::<(char, char)>()
            .any(|p| p == ('a', 'b') || p == ('c', 'd') || p == ('p', 'q') || p == ('x', 'y'))
}

fn nice2(s: &str) -> bool {
    let mut pairs = HashMap::new();
    let mut found = false;
    for (i, (a, b)) in s.chars().tuple_windows().enumerate() {
        if let Some(j) = pairs.get(&(a, b)) {
            if i > *j + 1 {
                found = true;
                break;
            }
        } else {
            pairs.insert((a, b), i);
        }
    }
    found
        && s.chars()
            .tuple_windows::<(char, char, char)>()
            .any(|(a, _b, c)| a == c)
}

fn main() {
    let words = std::io::stdin()
        .lock()
        .lines()
        .map(|r| r.unwrap())
        .collect::<Vec<String>>();

    println!("Part 1: {}", words.iter().filter(|s| nice1(s)).count());
    println!("Part 2: {}", words.iter().filter(|s| nice2(s)).count());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nice1() {
        assert!(nice1("ugknbfddgicrmopn"));
        assert!(nice1("aaa"));
        assert!(!nice1("jchzalrnumimnmhp"));
        assert!(!nice1("haegwjzuvuyypxyu"));
        assert!(!nice1("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_nice2() {
        assert!(nice2("qjhvhtzxzqqjkmpb"));
        assert!(nice2("xxyxx"));
        assert!(!nice2("uurcxstgmygtbstg"));
        assert!(!nice2("ieodomkazucvgmuy"));
    }
}
