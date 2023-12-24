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
    weights: &[Vec<Link>],
    seen: &mut [bool],
) -> Option<usize> {
    if pos == end {
        return Some(0);
    }
    assert!(!seen[pos]);
    seen[pos] = true;

    let mut best = None;
    for link in weights[pos].iter() {
        if seen[link.dst] {
            continue;
        }
        if let Some(w) = recurse(link.dst, end, weights, seen) {
            let w = w + link.steps;
            if let Some(prev) = best {
                if w > prev {
                    best = Some(w);
                }
            } else {
                best = Some(w);
            }
        }
    }
    assert!(seen[pos]);
    seen[pos] = false;
    best
}

/// Find the (single) path from `pos` to `end`
fn search_from_node(
    map: &HashSet<(i64, i64)>,
    nodes: &HashSet<(i64, i64)>,
    pos: (i64, i64),
) -> Vec<((i64, i64), usize)> {
    let mut next = vec![];
    for (dx, dy) in DIRECTIONS {
        let n = (pos.0 + dx, pos.1 + dy);
        if map.contains(&n) {
            next.push(n);
        }
    }
    assert!(next.len() == 1 || next.len() > 2);
    let mut out = HashMap::new();
    for n in next {
        if let Some((end, steps)) = explore_from_node(map, nodes, pos, n) {
            let prev = out.insert(end, steps + 1);
            assert!(prev.is_none());
        }
    }
    out.into_iter().collect()
}

fn explore_from_node(
    map: &HashSet<(i64, i64)>,
    nodes: &HashSet<(i64, i64)>,
    mut prev: (i64, i64),
    mut pos: (i64, i64),
) -> Option<((i64, i64), usize)> {
    for steps in 0.. {
        if nodes.contains(&pos) {
            return Some((pos, steps));
        }
        let (x, y) = pos;
        let mut next = None;
        for (dx, dy) in DIRECTIONS {
            let n = (x + dx, y + dy);
            if map.contains(&n) && n != prev {
                assert!(next.is_none());
                next = Some(n);
            }
        }
        let next = next.unwrap();
        prev = pos;
        pos = next;
    }
    unreachable!();
}

#[derive(Copy, Clone, Debug)]
struct Link {
    dst: usize,
    steps: usize,
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
    let mut nodes: HashSet<(i64, i64)> = map
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
    assert!(!nodes.contains(&start));
    assert!(!nodes.contains(&end));

    nodes.insert(start);
    nodes.insert(end);

    let mut connections = HashMap::new();
    for &a in &nodes {
        let conn = search_from_node(&map, &nodes, a);
        connections.insert(a, conn);
    }

    // Convert from (i64, i64) -> usize so that we can use flat Vecs
    let index = nodes
        .iter()
        .enumerate()
        .map(|(i, pos)| (*pos, i))
        .collect::<HashMap<(i64, i64), usize>>();

    let start = *index.get(&start).unwrap();
    let end = *index.get(&end).unwrap();

    let mut cs = Vec::new();
    cs.resize_with(nodes.len(), Vec::new);
    for (start, c) in connections {
        cs[index[&start]] = c
            .iter()
            .map(|(pos, steps)| Link {
                dst: index[&pos],
                steps: *steps,
            })
            .collect();
    }

    let p2 = recurse(start, end, &cs, &mut vec![false; nodes.len()]).unwrap();

    // 6593 is too high
    (p1.to_string(), p2.to_string())
}
