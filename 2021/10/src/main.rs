use std::io::BufRead;

fn parse(s: &str) -> Result<usize, usize> {
    let mut stack = vec![];
    for c in s.chars() {
        match (stack.last().unwrap_or(&' '), c) {
            // Opening is always allowed
            (_, '(' | '[' | '{' | '<') => {
                stack.push(c);
            }
            // Check for valid closing chars
            ('(', ')') | ('[', ']') | ('{', '}') | ('<', '>') => {
                stack.pop();
            }
            // Otherwise, it's an invalid closing char
            (_, ')') => return Err(3),
            (_, ']') => return Err(57),
            (_, '}') => return Err(1197),
            (_, '>') => return Err(25137),
            _ => panic!("Invalid input {}", s),
        }
    }
    Ok(stack.iter().rev().fold(0, |score, c| {
        score * 5
            + match c {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => unreachable!(),
            }
    }))
}

fn main() {
    let scores = std::io::stdin()
        .lock()
        .lines()
        .map(|line| parse(&line.unwrap()))
        .collect::<Vec<_>>();

    println!(
        "Part 1: {}",
        scores.iter().filter_map(|s| s.err()).sum::<usize>()
    );
    let mut scores = scores
        .into_iter()
        .filter_map(|s| s.ok())
        .collect::<Vec<usize>>();
    scores.sort_unstable();
    println!("Part 2: {}", scores[scores.len() / 2]);
}
