use std::io::BufRead;

fn decode(s: &str) -> usize {
    let mut escaping = false;
    let mut hex = 0;
    let mut bytes = 0;
    for c in s.chars() {
        if hex > 0 {
            hex -= 1;
        } else if escaping {
            if c == 'x' {
                hex = 2;
            }
            bytes += 1;
            escaping = false;
        } else if c == '\\' {
            escaping = true;
        } else if c != '"' {
            bytes += 1;
        }
    }
    s.len() - bytes
}

fn encode(s: &str) -> usize {
    s.matches('"').count() +
    s.matches('\\').count() + 2
}

fn main() {
    let words = std::io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap())
        .collect::<Vec<String>>();

    let score: usize = words.iter()
        .map(|line| decode(&line))
        .sum();
    println!("Part 1: {}", score);

    let score: usize = words.iter()
        .map(|line| encode(&line))
        .sum();
    println!("Part 2: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        assert_eq!(decode(r#""""#), 2);
        assert_eq!(decode(r#""abc""#), 2);
        assert_eq!(decode(r#""aaa\"aaa""#), 3);
        assert_eq!(decode(r#""\x27""#), 5);
    }

    #[test]
    fn test_encode() {
        assert_eq!(encode(r#""""#), 4);
        assert_eq!(encode(r#""abc""#), 4);
        assert_eq!(encode(r#""aaa\"aaa""#), 6);
        assert_eq!(encode(r#""\x27""#), 5);
    }
}
