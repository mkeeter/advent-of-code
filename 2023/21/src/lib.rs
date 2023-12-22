use std::collections::HashSet;

fn neighbors(pos: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
    let (x, y) = pos;
    let n = if y > 0 { Some((x, y - 1)) } else { None };
    let s = Some((x, y + 1));
    let w = if x > 0 { Some((x - 1, y)) } else { None };
    let e = Some((x + 1, y));
    [n, s, e, w].into_iter().flatten()
}

/// Returns the number of steps it takes to hit every tile from (0, 0)
fn sat_steps(map: &HashSet<(usize, usize)>) -> usize {
    sat_steps_from(map, (0, 0))
}

fn sat_steps_from(
    map: &HashSet<(usize, usize)>,
    start: (usize, usize),
) -> usize {
    let mut pos: HashSet<(usize, usize)> = [start].into_iter().collect();
    let mut seen: HashSet<(usize, usize)> = pos.clone();
    for i in 0.. {
        let mut next = HashSet::new();
        for p in pos.into_iter() {
            for n in neighbors(p) {
                if map.contains(&n) && seen.insert(n) {
                    next.insert(n);
                }
            }
        }
        pos = next;
        if pos.is_empty() {
            return i;
        }
    }
    panic!("oh no");
}

/// Returns the number of tiles filled in even and odd grids respectively
fn sat_even_odd(
    map: &HashSet<(usize, usize)>,
    start: (usize, usize),
) -> (usize, usize) {
    let mut pos: HashSet<(usize, usize)> = [start].into_iter().collect();
    let mut seen: HashSet<(usize, usize)> = pos.clone();
    let mut sat_even = 0;
    let mut sat_odd = 0;
    for i in 0.. {
        let mut next = HashSet::new();
        if i % 2 == 0 {
            sat_even += pos.len();
        } else {
            sat_odd += pos.len();
        }
        for p in pos.into_iter() {
            for n in neighbors(p) {
                if map.contains(&n) && seen.insert(n) {
                    next.insert(n);
                }
            }
        }
        pos = next;
        if pos.is_empty() {
            break;
        }
    }
    (sat_even, sat_odd)
}

/// Returns the number of steps to get from `start` to (0, 0)
///
/// We exit through the south-east (+ve, +ve) corner of the grid, which is the
/// point farthest from (0, 0)
fn steps_to_corner(
    map: &HashSet<(usize, usize)>,
    size: usize,
    start: (usize, usize),
) -> usize {
    let mut pos: HashSet<(usize, usize)> = [start].into_iter().collect();
    let mut seen: HashSet<(usize, usize)> = pos.clone();
    for i in 0.. {
        let mut next = HashSet::new();
        for p in pos.into_iter() {
            if p.0 == size - 1 && p.1 == size - 1 {
                return i + 2;
            }

            for n in neighbors(p) {
                if map.contains(&n) && seen.insert(n) {
                    next.insert(n);
                }
            }
        }
        pos = next;
        assert!(!pos.is_empty());
    }
    panic!("could not reach corner");
}

fn run_corner(
    map: &HashSet<(usize, usize)>,
    max_steps: usize,
    size: usize,
    start: (usize, usize),
) -> usize {
    assert_eq!(start.0, start.1);
    assert_eq!(start.0 * 2 + 1, size);

    // Find how many steps it takes to get from the start to the corner
    let corner_steps = steps_to_corner(map, size, start);
    assert_eq!(corner_steps, size - 1 + 2);

    let mut remaining_steps = max_steps;
    remaining_steps -= corner_steps;

    let (sat_even, sat_odd) = sat_even_odd(map, start);
    let (sat_even, sat_odd) = if remaining_steps % 2 == 1 {
        (sat_odd, sat_even)
    } else {
        (sat_even, sat_odd)
    };

    // Find out how many steps it takes to saturate the map starting from that
    // corner
    let sat_steps = sat_steps(map);

    // Build up the bulk of the triangle
    //
    // O        grid = 0, 1 partial
    //
    // Ep       grid = 1, 2 partial, 1 even
    // p
    //
    // EOp      grid = 2, 3 partial, 1 even, 2 odd
    // Op
    // p
    //
    // EOEp     grid = 3, 4 partial, 4 (1 + 3) even, 2 odd
    // OEp
    // Ep
    // p
    //
    // EOEOp    grid = 4, 5 partial, 4 (1 + 3) even, 6 (2 + 4) odd
    // EOEp
    // EOp
    // Ep
    // p

    let mut grid = 0;
    let mut count_even = 0;
    let mut count_odd = 0;
    while remaining_steps > sat_steps {
        if grid % 2 == 0 {
            count_even += grid + 1;
        } else {
            count_odd += grid + 1;
        }
        remaining_steps -= size;
        grid += 1;
    }

    // flood-fill remaining steps
    let mut pos: HashSet<(usize, usize)> = [(0, 0)].into_iter().collect();
    for _ in 0..remaining_steps {
        let mut next = HashSet::new();
        for p in pos.into_iter() {
            for n in neighbors(p) {
                if map.contains(&(n.0 % size, n.1 % size)) {
                    next.insert(n);
                }
            }
        }
        pos = next;
    }

    // Count points from the flood fill
    //
    // p-|x---->
    // --|-------
    // y | z
    // | |
    // V |
    //
    // y and z overlap with later cells and should only be counted once
    let mut np = 0;
    let mut nx = 0;
    let mut ny = 0;
    let mut nz = 0;
    for (px, py) in pos {
        if px < size && py < size {
            np += 1;
        } else if px >= size && py < size {
            nx += 1;
        } else if px < size && py >= size {
            ny += 1;
        } else {
            nz += 1;
        }
    }

    // Work out how many even and odd tiles are in this quadrant
    let partial_grids = grid + 1;
    count_even * sat_even
        + count_odd * sat_odd
        + (np + nx) * partial_grids
        + ny
        + nz
}

fn run_up(
    map: &HashSet<(usize, usize)>,
    max_steps: usize,
    size: usize,
    start: (usize, usize),
) -> usize {
    // Find startup steps from each X position to (X, 0) in the (0, +1) grid
    let mut pos: HashSet<(usize, usize)> = [start].into_iter().collect();
    let mut starting_steps = vec![None; size];
    for i in 0.. {
        let mut next = HashSet::new();
        for p in pos.into_iter() {
            for n in neighbors(p) {
                if map.contains(&n) {
                    next.insert(n);
                } else if n.1 == size && starting_steps[n.0].is_none() {
                    starting_steps[n.0] = Some(i + 1);
                }
            }
        }
        if starting_steps.iter().all(Option::is_some) {
            break;
        }
        pos = next;
    }
    let starting_steps: Vec<usize> =
        starting_steps.into_iter().map(Option::unwrap).collect();

    // Saturating steps for each X position
    let sat_steps: Vec<usize> =
        (0..size).map(|x| sat_steps_from(map, (x, 0))).collect();

    // Map from [src_x][dst_x] to minimum steps it takes to get there
    let mut metamap: Vec<Vec<usize>> = Vec::new();
    for x in 0..size {
        let start = (x, 0);
        let mut pos: HashSet<(usize, usize)> = [start].into_iter().collect();
        let mut found = vec![None; size];
        let mut seen = HashSet::new();
        for i in 0.. {
            let mut next = HashSet::new();
            for p in pos.into_iter() {
                if !seen.insert(p) {
                    continue;
                }
                for n in neighbors(p) {
                    if map.contains(&n) {
                        next.insert(n);
                    } else if n.1 == size && found[n.0].is_none() {
                        found[n.0] = Some(i + 1);
                    }
                }
            }
            if found.iter().all(Option::is_some) {
                break;
            }
            pos = next;
        }
        metamap.push(found.into_iter().map(Option::unwrap).collect());
    }

    // Running map from x position to number-of-steps it took to get there
    let mut pos = starting_steps;
    let mut manual_search = None;
    for grid in 1.. {
        if grid % 1000 == 0 {
            println!(
                "{} / {max_steps} {}",
                pos.iter().max().unwrap(),
                *pos.iter().max().unwrap() as f32 / max_steps as f32
            );
        }
        let mut next = vec![usize::MAX; size];
        for (new_x, n) in next.iter_mut().enumerate() {
            for (src_x, steps) in pos.iter().enumerate() {
                *n = (*n).min(steps + metamap[src_x][new_x]);
            }
        }
        if next
            .iter()
            .enumerate()
            .all(|(x, steps)| steps + sat_steps[x] > max_steps)
        {
            manual_search = Some((grid, pos));
            break;
        }
        pos = next;
    }

    let (grid, pts) = manual_search.unwrap();

    let mut todo: Vec<(usize, usize, usize)> = pts
        .into_iter()
        .enumerate()
        .map(|(x, steps)| (x, 0, steps))
        .filter(|(_, _, steps)| *steps <= max_steps)
        .collect();
    let mut ends = HashSet::new();
    let mut seen = HashSet::new();
    while let Some((x, y, steps)) = todo.pop() {
        if !seen.insert((x, y, steps)) {
            continue;
        }
        if steps == max_steps {
            ends.insert((x, y));
            continue;
        }
        for n in neighbors((x, y)) {
            if n.0 >= size || !map.contains(&(n.0, n.1 % size)) {
                continue;
            }
            todo.push((n.0, n.1, steps + 1));
        }
    }
    // 1 -> 0 even, 0 odd (1 partial)
    // 2 -> 0 even, 1 odd (1 partial)
    // 3 -> 1 even, 1 odd (1 partial)
    // 4 -> 1 even, 2 odd (1 partial)
    // 5 -> 2 even, 2 odd (1 partial)
    //
    let num_even_grids = (grid - 1) / 2;
    let num_odd_grids = grid / 2;

    let (sat_even, sat_odd) = sat_even_odd(map, start);
    ends.len() + num_even_grids * sat_even + num_odd_grids * sat_odd
}

fn search_from(
    map: &HashSet<(usize, usize)>,
    start: (usize, usize),
    max_steps: usize,
) -> usize {
    let mut pos: HashSet<(usize, usize)> = [start].into_iter().collect();
    for _ in 0..max_steps {
        let mut next = HashSet::new();
        for p in pos.into_iter() {
            for n in neighbors(p) {
                if map.contains(&n) {
                    next.insert(n);
                }
            }
        }
        pos = next;
    }
    pos.len()
}

pub fn solve(s: &str) -> (String, String) {
    let mut map = HashSet::new();
    let mut start = None;
    let height = s.lines().count();
    let width = s.lines().next().unwrap().len();
    assert_eq!(width, height);
    let size = width;

    for (y, line) in s.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            // The map contains valid empty squares
            match c {
                '#' => (),
                '.' => {
                    map.insert((x, y));
                }
                'S' => {
                    assert!(start.is_none());
                    start = Some((x, y));
                    map.insert((x, y));
                }
                c => panic!("invalid character {c}"),
            }
        }
    }
    let start = start.unwrap();
    let p1 = search_from(&map, start, 64);

    ////////////////////////////////////////////////////////////////////////////

    const MAX_STEPS: usize = 26501365;
    let mut rot = map.clone();
    let mut sum = 0;
    for _ in 0..4 {
        let corner = run_corner(&rot, MAX_STEPS, size, start);
        sum += corner;

        let up = run_up(&rot, MAX_STEPS, size, start);
        sum += up;

        rot = rot.into_iter().map(|(x, y)| (size - y - 1, x)).collect();
    }
    let base = sat_even_odd(&map, start);
    let base = if MAX_STEPS % 2 == 0 { base.0 } else { base.1 };
    sum += base; // TODO this is a hack, only valid for > saturating steps

    let p2 = sum;

    (p1.to_string(), p2.to_string())
}
