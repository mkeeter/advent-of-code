use std::collections::{HashMap, HashSet};

const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn run(
    mut map: HashMap<(i64, i64), char>,
    mut pos: (i64, i64),
    end: (i64, i64),
) -> usize {
    for steps in 0.. {
        if pos == end {
            return steps;
        }
        let Some(c) = map.remove(&pos) else {
            return 0;
        };
        match c {
            '.' => {
                // search all four directions
                let mut next = vec![];
                let (x, y) = pos;
                for (dx, dy) in DIRECTIONS {
                    let n = (x + dx, y + dy);
                    if map.contains_key(&n) {
                        next.push(n);
                    }
                }
                match next.len() {
                    0 => return 0,      // we've reached a corner; give up
                    1 => pos = next[0], // keep looping without cloning
                    _ => {
                        return steps
                            + 1
                            + next
                                .into_iter()
                                .map(|p| {
                                    let map = map.clone();
                                    run(map, p, end)
                                })
                                .max()
                                .unwrap_or(0);
                    }
                }
            }
            '>' => pos.0 += 1,
            '<' => pos.0 -= 1,
            '^' => pos.1 -= 1,
            'v' => pos.1 += 1,
            c => panic!("invalid character {c}"),
        }
    }
    unreachable!()
}

fn recurse(
    pos: usize,
    end: usize,
    weights: &[Vec<Option<usize>>],
    seen: &mut [bool],
) -> usize {
    if pos == end {
        return 0;
    }
    seen[pos] = true;
    let mut best = 0;
    for (j, steps) in weights[pos].iter().enumerate() {
        if seen[j] {
            continue;
        }
        let Some(steps) = steps else {
            continue;
        };
        let w = recurse(j, end, weights, seen) + steps;
        best = best.max(w);
    }
    seen[pos] = false;
    best
}

/// Find the (single) path from `pos` to `end`
fn search_for(
    map: &HashSet<(i64, i64)>,
    pos: (i64, i64),
    end: (i64, i64),
) -> Option<usize> {
    let mut next = vec![];
    for (dx, dy) in DIRECTIONS {
        let n = (pos.0 + dx, pos.1 + dy);
        if map.contains(&n) {
            next.push(n);
        }
    }
    assert!(!next.is_empty());
    let mut out = None;
    for n in next {
        if let Some(v) = search_for_inner(map, pos, n, end) {
            assert!(out.is_none());
            out = Some(v + 1)
        }
    }
    out
}

fn search_for_inner(
    map: &HashSet<(i64, i64)>,
    mut prev: (i64, i64),
    mut pos: (i64, i64),
    end: (i64, i64),
) -> Option<usize> {
    for steps in 0.. {
        if pos == end {
            return Some(steps);
        }

        let (x, y) = pos;
        let mut next = None;
        for (dx, dy) in DIRECTIONS {
            let n = (x + dx, y + dy);
            if map.contains(&n) && n != prev {
                if next.is_some() {
                    return None; // hit a node
                }
                next = Some(n);
            }
        }
        let Some(next) = next else {
            return None;
        };
        prev = pos;
        pos = next;
    }
    unreachable!()
}

pub fn solve(s: &str) -> (String, String) {
    let mut map = HashMap::new();
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '#' {
                map.insert((x as i64, y as i64), c);
            }
        }
    }
    let start = *map.keys().find(|(_, y)| *y == 0).unwrap();
    let end = *map.keys().max_by_key(|(_, y)| *y).unwrap();

    let p1 = run(map.clone(), start, end);

    let map = map.into_keys().collect::<HashSet<_>>();
    println!("map has {} tiles", map.len());
    let mut nodes: Vec<(i64, i64)> = map
        .iter()
        .cloned()
        .filter(|(x, y)| {
            let n = DIRECTIONS
                .iter()
                .filter(|(dx, dy)| map.contains(&(x + dx, y + dy)))
                .count();
            match n {
                0 => panic!("unconnected node"),
                1 | 2 => false, // dead-end or normal path
                3 | 4 => true,  // node!
                _ => panic!("superconnected node"),
            }
        })
        .collect();
    nodes.sort();
    nodes.dedup();
    assert!(!nodes.contains(&start));
    assert!(!nodes.contains(&end));

    nodes.insert(0, start);
    nodes.push(end);

    let start = nodes.iter().position(|n| *n == start).unwrap();
    let end = nodes.iter().position(|n| *n == end).unwrap();

    let mut weights = vec![];
    for a in &nodes {
        let mut ws = vec![];
        for b in &nodes {
            let r = search_for(&map, *a, *b);
            println!("{a:?} -> {b:?}: {r:?}");
            ws.push(r);
        }
        weights.push(ws);
    }
    let p2 = recurse(start, end, &weights, &mut vec![false; nodes.len()]);

    // 6593 is too high
    (p1.to_string(), p2.to_string())
}
