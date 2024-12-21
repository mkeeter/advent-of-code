use std::collections::{HashMap, VecDeque};
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
    (0..3).contains(&x) && (0..4).contains(&y) && !(x == 0 && y == 3)
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
    (0..3).contains(&x) && (0..2).contains(&y) && !(x == 0 && y == 0)
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

////////////////////////////////////////////////////////////////////////////////

/// Takes a chunk of directions and returns the length of the minimum expansion
///
/// The chunk has an implicit trailing `A`
fn expand_chunk<'a>(
    chunk: &'a str,
    depth: usize,
    dir_cache: &'a HashMap<(char, char), Vec<String>>,
    seen: &mut HashMap<(&'a str, usize), usize>,
) -> usize {
    assert!(!chunk.contains('A'));
    if let Some(v) = seen.get(&(chunk, depth)) {
        *v
    } else if depth == 0 {
        chunk.len() + 1 // for the trailing `A`
    } else {
        let length = std::iter::zip(
            std::iter::once('A').chain(chunk.chars()),
            chunk.chars().chain(std::iter::once('A')),
        )
        .map(|(a, b)| {
            dir_cache[&(a, b)]
                .iter()
                .map(|p| expand_chunk(p, depth - 1, dir_cache, seen))
                .min()
                .unwrap()
        })
        .sum();
        seen.insert((chunk, depth), length);
        length
    }
}

fn run<'a>(
    line: &str,
    depth: usize,
    np_cache: &'a HashMap<(char, char), Vec<String>>,
    dir_cache: &'a HashMap<(char, char), Vec<String>>,
    seen: &mut HashMap<(&'a str, usize), usize>,
) -> usize {
    let min_length: usize =
        std::iter::zip(std::iter::once('A').chain(line.chars()), line.chars())
            .map(|(a, b)| {
                np_cache[&(a, b)]
                    .iter()
                    .map(|chunk| expand_chunk(chunk, depth, dir_cache, seen))
                    .min()
                    .unwrap()
            })
            .sum();
    let v = line[0..3].parse::<usize>().unwrap();
    min_length * v
}

pub fn solve(s: &str) -> (usize, usize) {
    let mut np_cache = HashMap::new();
    const NUMS: &str = "0123456789A";
    for a in NUMS.chars() {
        for b in NUMS.chars() {
            np_cache.insert((a, b), num_paths(a, b));
        }
    }
    const DIRS: &str = "^v<>A";
    let mut dir_cache = HashMap::new();
    for a in DIRS.chars() {
        for b in DIRS.chars() {
            dir_cache.insert((a, b), dir_paths(a, b));
        }
    }

    let mut seen = HashMap::new();
    let p1 = s
        .lines()
        .map(|line| run(line, 2, &np_cache, &dir_cache, &mut seen))
        .sum();
    let p2 = s
        .lines()
        .map(|line| run(line, 25, &np_cache, &dir_cache, &mut seen))
        .sum();

    (p1, p2)
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
