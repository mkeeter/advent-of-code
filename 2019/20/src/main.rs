use std::io::BufRead;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let mut tiles = HashMap::new();
    for (y, line) in std::io::stdin().lock().lines().enumerate() {
        for (x, c) in line.unwrap().chars().enumerate() {
            tiles.insert((x as i32 + 1, y as i32 + 1), c);
        }
    }

    let get = |x: i32, y: i32| { *tiles.get(&(x, y)).unwrap_or(&'#') };

    let mut portals = tiles.iter()
        .filter(|(_k, v)| **v == '.')
        .flat_map(|(k, _v)| [(0, 1), (0, -1), (1, 0), (-1, 0)]
                  .iter()
                  .map(move |d| (*k, *d)))
        .filter_map(|((x, y), (dx, dy))| {
            let a = get(x +   dx, y +   dy);
            let b = get(x + 2*dx, y + 2*dy);
            if char::is_uppercase(a) && char::is_uppercase(b) {
                let key = if dx < 0 || dy < 0 {
                    [b, a]
                } else {
                    [a, b]
                };
                Some((key, (x, y), (x + dx, y + dy)))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    portals.sort_unstable();

    let enter = portals[0].1;
    let exit = portals.pop().unwrap().1;

    let mut links = HashMap::new();
    for c in portals[1..].chunks_exact(2) {
        links.insert(c[0].2, c[1].1);
        links.insert(c[1].2, c[0].1);
    }

    let mut seen = HashSet::new();
    let mut todo = VecDeque::new();
    todo.push_back((enter.0, enter.1, 0));
    while let Some((x, y, steps)) = todo.pop_front() {
        if (x, y) == exit {
            println!("Part 1: {}", steps);
            break;
        } else if !seen.insert((x, y)) {
            continue;
        }
        let c = get(x, y);
        if char::is_uppercase(c) {
            if let Some(w) = links.get(&(x, y)) {
                todo.push_back((w.0, w.1, steps));
            }
        } else if c != '#' {
            // Take new steps
            for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                todo.push_back((x + dx, y + dy, steps + 1));
            }
        }
    }

    // Helper function to check portal direction
    let recurse = {
        let xmin = links.keys().map(|p| p.0).min().unwrap();
        let ymin = links.keys().map(|p| p.1).min().unwrap();
        let xmax = links.keys().map(|p| p.0).max().unwrap();
        let ymax = links.keys().map(|p| p.1).max().unwrap();
        move |x: i32, y: i32| {
            if x == xmin || x == xmax || y == ymin || y == ymax {
                -1
            } else {
                1
            }
        }
    };

    let mut seen = HashSet::new();
    let mut todo = VecDeque::new();
    todo.push_back((enter.0, enter.1, 0, 0));
    while let Some((x, y, steps, level)) = todo.pop_front() {
        if (x, y) == exit && level == 0 {
            println!("Part 2: {}", steps);
            break;
        } else if !seen.insert((x, y, level)) {
            continue;
        }
        let c = get(x, y);
        if char::is_uppercase(c) {
            if let Some(w) = links.get(&(x, y)) {
                todo.push_back((w.0, w.1, steps, level + recurse(x, y)));
            }
        } else if c != '#' && level >= 0 {
            // Take new steps
            for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                todo.push_back((x + dx, y + dy, steps + 1, level));
            }
        }
    }
}
