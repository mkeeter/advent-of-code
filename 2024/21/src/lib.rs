use std::collections::VecDeque;
use util::Dir;

fn num_pos(a: char) -> (i64, i64) {
    match a {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        c => panic!("invalid char '{c}'"),
    }
}

fn num_valid(x: i64, y: i64) -> bool {
    x >= 0 && x < 3 && y >= 0 && y < 4 && !(x == 0 && y == 3)
}

fn dir_pos(a: char) -> (i64, i64) {
    match a {
        '^' => (1, 0),
        'A' => (2, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        c => panic!("invalid char '{c}'"),
    }
}

fn dir_valid(x: i64, y: i64) -> bool {
    x >= 0 && x < 3 && y >= 0 && y < 2 && !(x == 0 && y == 0)
}

fn num_paths(a: char, b: char) -> Vec<String> {
    build_paths(a, b, num_pos, num_valid)
}

fn dir_paths(a: char, b: char) -> Vec<String> {
    build_paths(a, b, dir_pos, dir_valid)
}

fn build_paths(
    a: char,
    b: char,
    pos: fn(char) -> (i64, i64),
    valid: fn(i64, i64) -> bool,
) -> Vec<String> {
    let (sx, sy) = pos(a);
    let end = pos(b);
    let mut todo = VecDeque::new();
    todo.push_back((0, sx, sy, "".to_owned()));
    let mut best_dist = None;
    let mut paths = vec![];
    while let Some((t, x, y, s)) = todo.pop_front() {
        if best_dist.map(|b| t > b).unwrap_or(false) {
            continue;
        }
        if (x, y) == end {
            best_dist = Some(t);
            paths.push(s);
            continue;
        }
        for d in Dir::iter() {
            let x = x + d.x();
            let y = y + d.y();
            if valid(x, y) {
                let mut s = s.clone();
                s.push(char::from(d));
                todo.push_back((t + 1, x, y, s));
            }
        }
    }
    paths
}

pub fn solve(s: &str) -> (usize, u64) {
    let mut out = 0;
    for line in s.lines() {
        let mut paths = vec!["".to_owned()];
        let mut min_length = usize::MAX;
        for (a, b) in std::iter::zip(
            std::iter::once('A').chain(line.chars()),
            line.chars(),
        ) {
            let mut next = vec![];
            for d in num_paths(a, b) {
                for p in &paths {
                    let mut p = p.clone();
                    p += &d;
                    p += "A";
                    next.push(p);
                }
            }
            paths = next;
        }
        for line in paths {
            let mut paths = vec!["".to_owned()];
            for (a, b) in std::iter::zip(
                std::iter::once('A').chain(line.chars()),
                line.chars(),
            ) {
                let mut next = vec![];
                for d in dir_paths(a, b) {
                    for p in &paths {
                        let mut p = p.clone();
                        p += &d;
                        p += "A";
                        next.push(p);
                    }
                }
                paths = next;
            }
            for line in paths {
                let mut paths = vec!["".to_owned()];
                for (a, b) in std::iter::zip(
                    std::iter::once('A').chain(line.chars()),
                    line.chars(),
                ) {
                    let mut next = vec![];
                    for d in dir_paths(a, b) {
                        for p in &paths {
                            let mut p = p.clone();
                            p += &d;
                            p += "A";
                            next.push(p);
                        }
                    }
                    paths = next;
                }
                for p in paths {
                    min_length = min_length.min(p.len());
                }
            }
        }
        println!("{} x {}", min_length, s[0..3].parse::<usize>().unwrap());
        out += min_length * line[0..3].parse::<usize>().unwrap();
    }
    // there are some number of paths for the inner keypad
    // e.g. A -> 4
    // Then
    (out, 0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        const EXAMPLE: &str = indoc::indoc! {"
            029A
            980A
            179A
            456A
            379A
        "};
        assert_eq!(solve(EXAMPLE).0, 126384);
    }
}
