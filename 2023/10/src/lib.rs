use std::collections::{BTreeMap, BTreeSet};

fn neighbors(c: char) -> [(i32, i32); 2] {
    match c {
        '|' => [(0, -1), (0, 1)],
        '-' => [(-1, 0), (1, 0)],
        'F' => [(0, 1), (1, 0)],
        'J' => [(-1, 0), (0, -1)],
        'L' => [(1, 0), (0, -1)],
        '7' => [(-1, 0), (0, 1)],
        _ => panic!("Invalid pipe '{c}'"),
    }
}

pub fn solve(s: &str) -> (String, String) {
    let mut map = BTreeMap::new();
    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c != '.' {
                map.insert((x as i32, y as i32), c);
            }
        }
    }

    let start = *map.iter().find(|(_, c)| **c == 'S').unwrap().0;
    let c = "|-FJL7"
        .chars()
        .find(|c| {
            neighbors(*c).iter().all(|(dx, dy)| {
                let x = start.0 + dx;
                let y = start.1 + dy;
                if let Some(c) = map.get(&(x, y)) {
                    neighbors(*c)
                        .iter()
                        .any(|(nx, ny)| (x + nx, y + ny) == start)
                } else {
                    false
                }
            })
        })
        .unwrap();

    // Replace 'S' with the appropriate pipe symbol
    map.insert(start, c);

    let mut prev = [start, start];
    let mut next = neighbors(c).map(|(dx, dy)| (start.0 + dx, start.1 + dy));

    let mut path = BTreeSet::new();
    path.insert(start);

    let mut steps = 1;
    while next[0] != next[1] {
        path.extend(next.iter().cloned());
        let next_prev = next;
        for (prev, pos) in prev.iter().zip(next.iter_mut()) {
            for (dx, dy) in neighbors(*map.get(pos).unwrap()) {
                let x = pos.0 + dx;
                let y = pos.1 + dy;
                if (x, y) != *prev {
                    *pos = (x, y);
                    break;
                }
            }
        }
        prev = next_prev;
        steps += 1;
    }
    path.extend(next.iter().cloned());

    // Remove non-path tiles from the map
    map.retain(|k, _| path.contains(k));

    let mut xmin = i32::MAX;
    let mut xmax = i32::MIN;
    let mut ymin = i32::MAX;
    let mut ymax = i32::MIN;
    for (x, y) in map.keys() {
        xmin = xmin.min(*x);
        xmax = xmax.max(*x);
        ymin = ymin.min(*y);
        ymax = ymax.max(*y);
    }
    let mut inside = 0;
    for x in xmin..=xmax {
        let mut winding = 0;
        for y in ymin..=ymax {
            if let Some(c) = map.get(&(x, y)) {
                winding ^= match c {
                    'F' | 'L' => 0b01,
                    'J' | '7' => 0b10,
                    '-' => 0b11,
                    _ => 0b00,
                };
            } else {
                match winding {
                    0b11 => inside += 1,
                    0b00 => (),
                    c => panic!("invalid winding {c:02b}"),
                }
            }
        }
    }
    (steps.to_string(), inside.to_string())
}
