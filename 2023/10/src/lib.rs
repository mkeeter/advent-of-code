use std::collections::{BTreeMap, BTreeSet};

fn neighbors(c: char) -> [(i64, i64); 2] {
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
    let mut map = util::DenseGrid::new(s);

    let start = map.iter().find(|(_, c)| **c == 'S').unwrap().0;
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
    let mut pos = neighbors(c).map(|(dx, dy)| (start.0 + dx, start.1 + dy));

    let mut path = BTreeSet::new();
    path.insert(start);

    let mut steps = 1;
    while pos[0] != pos[1] {
        path.extend(pos.iter().cloned());
        let next_prev = pos;
        for (prev, pos) in prev.iter().zip(pos.iter_mut()) {
            *pos = neighbors(*map.get(pos).unwrap())
                .map(|(dx, dy)| (pos.0 + dx, pos.1 + dy))
                .into_iter()
                .find(|p| p != prev)
                .unwrap();
        }
        prev = next_prev;
        steps += 1;
    }
    path.extend(pos.iter().cloned());

    // Remove non-path tiles from the map
    map.retain(|k, _| path.contains(k));

    let bounds = map.bounds();
    let mut inside = 0;
    for y in bounds.ymin..=bounds.ymax {
        let mut winding = 0;
        for x in bounds.xmin..=bounds.xmax {
            if let Some(c) = map.get(&(x, y)) {
                winding ^= match c {
                    'F' | '7' => 0b01,
                    'J' | 'L' => 0b10,
                    '|' => 0b11,
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
