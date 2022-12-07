use std::collections::{HashMap, HashSet};
use std::io::Read;

fn run(mut s: &str) -> (i32, i32) {
    let mut pos = (0, 0);
    while !s.is_empty() {
        let prefix = if s.len() >= 2 { &s[..2] } else { "" };
        let (s_, d) = match prefix {
            "ne" => (&s[2..], (0, 1)),
            "nw" => (&s[2..], (-1, 1)),
            "se" => (&s[2..], (1, -1)),
            "sw" => (&s[2..], (0, -1)),
            _ => match &s[..1] {
                "e" => (&s[1..], (1, 0)),
                "w" => (&s[1..], (-1, 0)),
                _ => panic!("Invalid s: {}", s),
            },
        };
        s = s_;
        pos = (pos.0 + d.0, pos.1 + d.1)
    }
    pos
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut black = HashSet::new();
    for line in input.lines() {
        let pos = run(line);
        if black.contains(&pos) {
            black.remove(&pos);
        } else {
            black.insert(pos);
        }
    }
    println!("Part 1: {}", black.len());

    const NEIGHBORS: [(i32, i32); 6] = [(0, 1), (0, -1), (1, 0), (-1, 0), (-1, 1), (1, -1)];
    for _i in 0..100 {
        let mut count: HashMap<(i32, i32), usize> = HashMap::new();
        black
            .iter()
            .flat_map(|(x, y)| NEIGHBORS.iter().map(move |(dx, dy)| (x + dx, y + dy)))
            .for_each(|(x, y)| {
                count.entry((x, y)).and_modify(|i| *i += 1).or_insert(1);
            });
        black = count
            .into_iter()
            .filter(|((x, y), n)| (black.contains(&(*x, *y)) && !(*n == 0 || *n > 2)) || *n == 2)
            .map(|(pos, _n)| pos)
            .collect();
    }
    println!("Part 2: {}", black.len());
}
